use crate::command::command::{
    Argument, Command, CommandParseError, ParsedArgument, ParsedCommand,
};
use crate::database::Database;
use crate::{
    command::{context::Context, registry::CommandRegistry},
    util::regexes,
};
use bytes::Bytes;
use reqwest::{Client as ReqwestClient, StatusCode};
use serde::Deserialize;
use tokio::sync::RwLock;
use std::sync::Arc;
use std::{fs::read_to_string, time::Instant};
use twilight_http::Client as HttpClient;
use twilight_model::channel::Message;

#[derive(Clone, Deserialize)]
struct DatabaseInfo {
    username: Box<str>,
    password: Box<str>,
    host: Box<str>,
    port: u16,
    database: Box<str>,
}
impl DatabaseInfo {
    pub fn to_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }
}
#[derive(Clone, Deserialize)]
pub struct Config {
    database: DatabaseInfo,
    pub default_prefix: Box<str>,
}
impl Config {
    fn new() -> Self {
        let buffer = read_to_string("./config.toml").unwrap();
        toml::from_str(&buffer).unwrap()
    }
}

pub fn get_command_name_from<'a>(content: &'a str, prefix: &str) -> Option<&'a str> {
    return if !content.starts_with(&prefix) || content.len() == prefix.len() {
        None
    } else {
        Some(&content.split_whitespace().collect::<Vec<&str>>()[0][prefix.len()..])
    };
}

pub struct Assyst {
    pub registry: CommandRegistry,
    pub config: Config,
    pub database: Database,
    pub http: HttpClient,
    pub reqwest_client: ReqwestClient,
}
impl Assyst {
    pub async fn new(token: &str) -> Self {
        let http = HttpClient::new(token);
        let config = Config::new();
        let database = Database::new(2, config.database.to_url()).await.unwrap();
        let mut assyst = Assyst {
            registry: CommandRegistry::new(),
            config,
            database,
            http,
            reqwest_client: ReqwestClient::new(),
        };
        assyst.registry.register_commands();
        assyst
    }

    pub async fn handle_command(self: &Arc<Self>, message: Message) -> Result<(), String> {
        let try_prefix = self
            .database
            .get_or_set_prefix_for(message.guild_id.unwrap().0, &self.config.default_prefix)
            .await
            .map_err(|err| err.to_string())?;
        let prefix;

        match try_prefix {
            Some(p) => {
                prefix = p;
            }
            None => return Ok(()),
        };

        let t_command = self.parse_command(&message, &prefix).await;
        let context = Arc::new(Context::new(self.clone(), message));
        let command = match t_command {
            Ok(res) => match res {
                Some(c) => c,
                None => return Ok(()),
            },
            Err(e) => {
                if e.should_reply {
                    context
                        .reply_err(&e.error)
                        .await
                        .map_err(|e| e.to_string())?;
                }
                return Ok(());
            }
        };
        let context_clone = context.clone();
        let command_result = self.registry.execute_command(command, context_clone).await;
        match command_result {
            Err(err) => {
                context.reply_err(&err).await.map_err(|e| e.to_string())?;
            }
            Ok(_) => {}
        };
        Ok(())
    }

    pub async fn parse_command(
        &self,
        message: &Message,
        prefix: &str,
    ) -> Result<Option<ParsedCommand>, CommandParseError> {
        let content = &message.content;
        let command_name = get_command_name_from(&content, &prefix).unwrap_or("");
        let try_command = self.registry.get_command_from_name_or_alias(command_name);
        let command = match try_command {
            Some(c) => c,
            None => return Ok(None),
        };
        let mut args: Vec<&str> = content.split_whitespace().collect();
        let mut parsed_args: Vec<ParsedArgument> = vec![];
        args.remove(0);
        let mut index = 0;
        for arg in &command.args {
            match arg {
                Argument::ImageUrl => {
                    let argument_to_pass = if args.len() <= index { "" } else { args[index] };
                    let try_url = self.parse_image_argument_as_url(message, argument_to_pass).await;
                    if try_url == None {
                        return Err(CommandParseError::with_reply("This command expects an image as an argument, but no image could be found.".to_owned()))
                    } else {
                        parsed_args.push(ParsedArgument::Text(try_url.unwrap()));
                    }
                },
                Argument::ImageBuffer => {
                    let argument_to_pass = if args.len() <= index { "" } else { args[index] };
                    let try_url = self.parse_image_argument_as_bytes(message, argument_to_pass).await;
                    if try_url == None {
                        return Err(CommandParseError::with_reply("This command expects an image as an argument, but no image could be found.".to_owned()))
                    } else {
                        parsed_args.push(ParsedArgument::Binary(try_url.unwrap()));
                    }
                },
                Argument::String => {
                    parsed_args.push(ParsedArgument::Text(args[index].to_owned()));
                    index += 1;
                },
                _ => {}
            }
        }
        Ok(Some(ParsedCommand {
            calling_name: command.name.clone(),
            args: parsed_args,
        }))
    }

    pub async fn parse_image_argument_as_bytes(
        &self,
        message: &Message,
        argument: &str
    ) -> Option<Bytes> {
        let result = self
            .parse_image_argument(message, argument, &Argument::ImageBuffer)
            .await?;
        if let ParsedArgument::Binary(t) = result {
            Some(t)
        } else {
            None
        }
    }

    pub async fn parse_image_argument_as_url(
        &self,
        message: &Message,
        argument: &str
    ) -> Option<String> {
        let result = self
            .parse_image_argument(message, argument, &Argument::ImageUrl)
            .await?;
        if let ParsedArgument::Text(t) = result {
            Some(t)
        } else {
            None
        }
    }

    async fn parse_image_argument(
        &self,
        message: &Message,
        argument: &str,
        return_as: &Argument
    ) -> Option<ParsedArgument> {
        let emoji_url = self.validate_emoji_url(argument).await?;
        return match return_as {
            Argument::ImageBuffer => {
                let result = self.reqwest_client.get(&emoji_url).send().await.ok()?;
                if result.status() != StatusCode::OK {
                    None
                } else {
                    let bytes = result.bytes().await.ok()?;
                    Some(ParsedArgument::Binary(bytes))
                }
            }
            Argument::ImageUrl => Some(ParsedArgument::Text(emoji_url)),
            _ => panic!("return_as must be imageurl or imagebuffer"),
        };
    }

    async fn validate_emoji_url(&self, argument: &str) -> Option<String> {
        let emoji_id = regexes::CUSTOM_EMOJI
            .captures(argument)
            .and_then(|emoji_id_capture| emoji_id_capture.get(1))
            .and_then(|id| Some(id.as_str()))
            .and_then(|id| id.parse::<u64>().ok())?;

        let emoji_url = format!("https://cdn.discordapp.com/emojis/{}", emoji_id);

        return Some(emoji_url);
    }
}
