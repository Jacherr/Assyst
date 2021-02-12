use crate::command::command::{
    Argument, Command, CommandParseError, ParsedArgument, ParsedArgumentResult, ParsedCommand,
};
use crate::{
    badtranslator,
    caching::{Replies, Reply},
    command::context::Metrics,
    database::Database,
};
use crate::{badtranslator::BadTranslator, command::command::CommandAvailability};
use crate::{
    command::{context::Context, registry::CommandRegistry},
    util::regexes,
};
use reqwest::{Client as ReqwestClient, StatusCode};
use serde::Deserialize;
use std::fs::read_to_string;
use std::{borrow::Borrow, sync::Arc};
use std::{borrow::Cow, time::Instant};
use tokio::sync::RwLock;
use twilight_http::Client as HttpClient;
use twilight_model::channel::Message;
use twilight_model::id::UserId;
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
    pub admins: Vec<u64>,
    database: DatabaseInfo,
    pub default_prefix: Box<str>,
    pub disable_bad_translator: bool,
    pub prefix_override: Box<str>,
    pub wsi_url: Box<str>,
    pub wsi_auth: Box<str>
}
impl Config {
    fn new() -> Self {
        let buffer = read_to_string("./config.toml").unwrap();
        toml::from_str(&buffer).unwrap()
    }
}

pub fn get_command_and_args<'a>(content: &'a str, prefix: &str) -> Option<(&'a str, Vec<&'a str>)> {
    return if !content.starts_with(&prefix) || content.len() == prefix.len() {
        None
    } else {
        let raw = &content[prefix.len()..].trim_start();
        let mut args = raw.split_whitespace().collect::<Vec<_>>();
        let cmd = args[0];
        args.remove(0);
        Some((cmd, args))
    };
}

pub struct Assyst {
    pub config: Config,
    pub database: Database,
    pub http: HttpClient,
    pub registry: CommandRegistry,
    pub replies: RwLock<Replies>,
    pub reqwest_client: ReqwestClient,
    pub badtranslator: BadTranslator,
}
impl Assyst {
    pub async fn new(token: &str) -> Self {
        let http = HttpClient::new(token);
        let config = Config::new();
        let database = Database::new(2, config.database.to_url()).await.unwrap();
        let mut assyst = Assyst {
            config,
            database,
            http,
            badtranslator: BadTranslator::new(),
            registry: CommandRegistry::new(),
            replies: RwLock::new(Replies::new()),
            reqwest_client: ReqwestClient::new(),
        };
        if assyst.config.disable_bad_translator { assyst.badtranslator.disable().await };
        assyst.registry.register_commands();
        assyst
    }

    pub async fn handle_command(self: &Arc<Self>, _message: Message) -> Result<(), String> {
        let start = Instant::now();
        let message = Arc::new(_message);
        let prefix;
        if self.config.prefix_override.len() == 0 {
            let try_prefix = self
                .database
                .get_or_set_prefix_for(message.guild_id.unwrap().0, &self.config.default_prefix)
                .await
                .map_err(|err| err.to_string())?;

            match try_prefix {
                Some(p) => {
                    prefix = p;
                }
                None => return Ok(()),
            };
        } else {
            prefix = Cow::Borrowed(&self.config.prefix_override);
        };

        if !message.content.starts_with(prefix.borrow() as &str) {
            return Ok(());
        };

        let mut replies = self.replies.write().await;
        let reply = replies.get_or_set_reply(Reply::new(message.clone())).clone();
        
        let mut reply_lock = reply.lock().await;
        if reply_lock.has_expired() || reply_lock.in_use { return Ok(()) };
        reply_lock.in_use = true;
        drop(reply_lock);
        drop(replies);

        let t_command = self.parse_command(&message, &prefix).await;
        let metrics = Metrics {
            processing_time_start: start,
        };
        let context = Arc::new(Context::new(
            self.clone(),
            message.clone(),
            metrics,
            reply.clone(),
        ));
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
        reply.lock().await.in_use = false;
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
        let command_details = get_command_and_args(&content, &prefix).unwrap_or(("", vec![]));
        let try_command = self
            .registry
            .get_command_from_name_or_alias(&command_details.0.to_ascii_lowercase());
        let command = match try_command {
            Some(c) => c,
            None => return Ok(None),
        };

        match command.availability {
            CommandAvailability::Public => Ok(()),
            CommandAvailability::Private => {
                if !self.config.admins.contains(&message.author.id.0) {
                    Err(CommandParseError::without_reply(
                        "Insufficient Permissions".to_owned(),
                    ))
                } else {
                    Ok(())
                }
            },
            _ => Err(CommandParseError::without_reply(
                "Insufficient Permissions".to_owned(),
            )),
        }?;

        let parsed_args = self
            .parse_arguments(&message, &command, command_details.1)
            .await?;
        Ok(Some(ParsedCommand {
            calling_name: &command.name,
            args: parsed_args,
        }))
    }

    async fn parse_arguments(
        &self,
        message: &Message,
        command: &Command,
        args: Vec<&str>,
    ) -> Result<Vec<ParsedArgument>, CommandParseError> {
        let mut parsed_args: Vec<ParsedArgument> = vec![];
        let mut index = 0;
        for arg in &command.args {
            match arg {
                Argument::ImageUrl | Argument::ImageBuffer => {
                    let argument_to_pass = if args.len() <= index { "" } else { args[index] };
                    let try_result = self
                        .parse_image_argument(message, argument_to_pass, arg)
                        .await;
                    if let Some(result) = try_result {
                        parsed_args.push(result.value);
                        if result.should_increment_index {
                            index += 1
                        };
                    } else {
                        return Err(CommandParseError::with_reply("This command expects an image as an argument, but no image could be found.".to_owned()));
                    }
                }
                Argument::String => {
                    if args.len() <= index {
                        return Err(CommandParseError::with_reply(
                            "This command expects a text argument that was not provided."
                                .to_owned(),
                        ));
                    }
                    parsed_args.push(ParsedArgument::Text(args[index].to_owned()));
                    index += 1;
                }
                Argument::StringRemaining => {
                    if args.len() <= index {
                        return Err(CommandParseError::with_reply(
                            "This command expects a text argument that was not provided."
                                .to_owned(),
                        ));
                    }
                    parsed_args.push(ParsedArgument::Text(args[index..].join(" ")));
                    break;
                }
            }
        }
        Ok(parsed_args)
    }

    async fn parse_image_argument(
        &self,
        message: &Message,
        argument: &str,
        return_as: &Argument,
    ) -> Option<ParsedArgumentResult> {
        let mut should_increment = true;
        let mut try_url = self.validate_emoji_argument(argument).await;
        if try_url.is_none() {
            try_url = self.validate_user_argument(argument).await
        };
        if try_url.is_none() {
            try_url = self.validate_url_argument(argument)
        };
        if try_url.is_none() {
            try_url = self.validate_message_attachment(message);
            if try_url.is_some() {
                should_increment = false
            };
        };
        if try_url.is_none() {
            try_url = self.validate_previous_message_attachment(message).await;
            if try_url.is_some() {
                should_increment = false
            };
        };

        let mut url = try_url?;
        if url.starts_with("https://tenor.com/view/") {
            let page = self.reqwest_client.get(&url).send().await.ok()?;
            let page_html = page.text().await.ok()?;
            let gif_url = regexes::TENOR_GIF
                .find(&page_html)
                .and_then(|url| Some(url.as_str()))?;
            url = gif_url.to_owned();
        };

        match return_as {
            Argument::ImageBuffer => {
                let result = self.reqwest_client.get(&url).send().await.ok()?;
                if result.status() != StatusCode::OK {
                    None
                } else {
                    let bytes = result.bytes().await.ok()?;
                    let parsed_argument_result = match should_increment {
                        true => ParsedArgumentResult::increment(ParsedArgument::Binary(bytes)),
                        false => ParsedArgumentResult::no_increment(ParsedArgument::Binary(bytes)),
                    };
                    Some(parsed_argument_result)
                }
            }
            Argument::ImageUrl => {
                let parsed_argument_result = match should_increment {
                    true => ParsedArgumentResult::increment(ParsedArgument::Text(url)),
                    false => ParsedArgumentResult::no_increment(ParsedArgument::Text(url)),
                };
                Some(parsed_argument_result)
            }
            _ => panic!("return_as must be imageurl or imagebuffer"),
        }
    }

    async fn validate_emoji_argument(&self, argument: &str) -> Option<String> {
        let emoji_id = regexes::CUSTOM_EMOJI
            .captures(argument)
            .and_then(|emoji_id_capture| emoji_id_capture.get(1))
            .and_then(|id| Some(id.as_str()))
            .and_then(|id| id.parse::<u64>().ok())?;

        let format = if argument.starts_with("<a") {
            "gif"
        } else {
            "png"
        };
        let emoji_url = format!("https://cdn.discordapp.com/emojis/{}.{}", emoji_id, format);

        return Some(emoji_url);
    }

    fn validate_message_attachment(&self, message: &Message) -> Option<String> {
        Some(message.attachments.first()?.proxy_url.clone())
    }

    async fn validate_previous_message_attachment(&self, message: &Message) -> Option<String> {
        let messages = self.http.channel_messages(message.channel_id).await.ok()?;
        let message_attachment_urls: Vec<Option<&String>> = messages
            .iter()
            .map(|message| {
                if let Some(embed) = message.embeds.first() {
                    embed.url.as_ref().or_else(|| {
                        embed
                            .image
                            .as_ref()
                            .and_then(|img| img.proxy_url.as_ref())
                            .or_else(|| {
                                embed
                                    .thumbnail
                                    .as_ref()
                                    .and_then(|thumbnail| thumbnail.proxy_url.as_ref())
                            })
                    })
                } else {
                    Some(&message.attachments.first()?.proxy_url)
                }
            })
            .collect();

        message_attachment_urls
            .iter()
            .find(|attachment| attachment.is_some())?
            .and_then(|x| Some(x.clone()))
    }

    fn validate_url_argument(&self, argument: &str) -> Option<String> {
        if regexes::URL.is_match(argument) {
            Some(argument.to_owned())
        } else {
            None
        }
    }

    async fn validate_user_argument(&self, argument: &str) -> Option<String> {
        let user_id = regexes::USER_MENTION
            .captures(argument)
            .and_then(|user_id_capture| user_id_capture.get(1))
            .and_then(|id| Some(id.as_str()))
            .and_then(|id| id.parse::<u64>().ok())?;

        let user = self.http.user(UserId::from(user_id)).await.ok()??;
        let avatar_hash = user.avatar;
        match avatar_hash {
            Some(hash) => {
                let format = if hash.starts_with("a_") { "gif" } else { "png" };
                Some(format!(
                    "https://cdn.discordapp.com/avatars/{}/{}.{}?size=1024",
                    user_id, hash, format
                ))
            }
            None => {
                let discrim = user.discriminator.parse::<u16>().ok()?;
                let avatar_number = discrim % 5;
                Some(format!(
                    "https://cdn.discordapp.com/embed/avatars/{}.png",
                    avatar_number
                ))
            }
        }
    }
}
