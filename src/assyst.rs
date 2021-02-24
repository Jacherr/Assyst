use crate::{badtranslator::BadTranslator, command::command::CommandAvailability};
use crate::{
    caching::{Replies, Reply},
    command::context::Metrics,
    database::Database,
};
use crate::{
    command::command::{
        force_as, Argument, Command, CommandParseError, ParsedArgument, ParsedArgumentResult,
        ParsedCommand,
    },
    metrics::GlobalMetrics,
};
use crate::{
    command::{context::Context, registry::CommandRegistry},
    consts::ABSOLUTE_INPUT_FILE_SIZE_LIMIT_BYTES,
    util::{download_content, regexes},
};
use bytes::Bytes;
use reqwest::Client as ReqwestClient;
use serde::Deserialize;
use std::{borrow::Borrow, sync::Arc};
use std::{borrow::Cow, time::Instant};
use std::{collections::HashSet, fs::read_to_string};
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
    pub admins: HashSet<u64>,
    database: DatabaseInfo,
    pub default_prefix: Box<str>,
    pub disable_bad_translator: bool,
    pub prefix_override: Box<str>,
    pub user_blacklist: HashSet<u64>,
    pub wsi_url: Box<str>,
    pub wsi_auth: Box<str>,
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
    pub metrics: RwLock<GlobalMetrics>,
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
            metrics: RwLock::new(GlobalMetrics::new()),
        };
        if assyst.config.disable_bad_translator {
            assyst.badtranslator.disable().await
        };
        assyst.registry.register_commands();
        assyst
    }

    pub async fn handle_command(self: &Arc<Self>, _message: Message) -> Result<(), String> {
        let start = Instant::now();
        let message = Arc::new(_message);
        if self.config.user_blacklist.contains(&message.author.id.0) {
            return Ok(());
        }
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
        let reply = replies
            .get_or_set_reply(Reply::new(message.clone()))
            .clone();

        let mut reply_lock = reply.lock().await;
        if reply_lock.has_expired() || reply_lock.in_use {
            return Ok(());
        };
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
                    let err = match e.command {
                        Some(c) => Cow::Owned(format!(
                            "{}\nUsage: {}{} {}",
                            e.error, prefix, c.name, c.metadata.usage
                        )),
                        None => Cow::Borrowed(&e.error),
                    };
                    context.reply_err(&err).await.map_err(|e| e.to_string())?;
                }
                return Ok(());
            }
        };

        let context_clone = context.clone();
        let command_result = self.registry.execute_command(command, context_clone).await;
        reply.lock().await.in_use = false;
        match command_result {
            Err(err) => {
                context
                    .reply_err(&err.replace("\\n", "\n"))
                    .await
                    .map_err(|e| e.to_string())?;
            }
            Ok(_) => {}
        };

        self.metrics
            .write()
            .await
            .processing
            .add(context.metrics.processing_time_start.elapsed().as_millis() as f32);

        Ok(())
    }

    pub async fn parse_command(
        &self,
        message: &Message,
        prefix: &str,
    ) -> Result<Option<ParsedCommand>, CommandParseError<'_>> {
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
            }
            CommandAvailability::GuildOwner => {
                let guild = self
                    .http
                    .guild(message.guild_id.unwrap())
                    .await
                    .map_err(|e| CommandParseError::with_reply(e.to_string(), None))?
                    .ok_or_else(|| {
                        CommandParseError::with_reply(
                            "Permission validator failed".to_owned(),
                            None,
                        )
                    })?;

                if guild.owner_id == message.author.id
                    || self.config.admins.contains(&message.author.id.0)
                {
                    Ok(())
                } else {
                    Err(CommandParseError::without_reply(
                        "Insufficient Permissions".to_owned(),
                    ))
                }
            }
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

    async fn parse_arguments<'a>(
        &self,
        message: &Message,
        command: &'a Command,
        args: Vec<&str>,
    ) -> Result<Vec<ParsedArgument>, CommandParseError<'a>> {
        let mut parsed_args: Vec<ParsedArgument> = vec![];
        let mut index = 0;
        for arg in &command.args {
            match arg {
                Argument::Choice(choices) => {
                    if args.len() <= index {
                        return Err(CommandParseError::with_reply(
                            format!("This command expects a choice argument (one of {:?}), but no argument was provided.", choices),
                                Some(command)
                        ));
                    }

                    let choice = match choices.iter().find(|&&choice| choice == args[index]) {
                        Some(k) => k,
                        None => {
                            return Err(CommandParseError::with_reply(
                                format!("Cannot find given argument in {:?}", choices),
                                Some(command),
                            ))
                        }
                    };

                    parsed_args.push(ParsedArgument::Choice(choice));

                    index += 1;
                }
                Argument::ImageUrl | Argument::ImageBuffer => {
                    let argument_to_pass = if args.len() <= index { "" } else { args[index] };
                    let try_result = self
                        .parse_image_argument(message, argument_to_pass, arg)
                        .await;

                    match try_result {
                        Ok(result) => {
                            parsed_args.push(result.value);
                            if result.should_increment_index {
                                index += 1
                            };
                        }
                        Err(mut e) => {
                            e.set_command(command);
                            return Err(e);
                        }
                    }
                }
                Argument::String => {
                    if args.len() <= index {
                        return Err(CommandParseError::with_reply(
                            "This command expects a text argument that was not provided."
                                .to_owned(),
                            Some(command),
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
                            Some(command),
                        ));
                    }
                    parsed_args.push(ParsedArgument::Text(args[index..].join(" ")));
                    break;
                }
            }
        }
        Ok(parsed_args)
    }

    async fn parse_image_argument<'a>(
        &self,
        message: &Message,
        argument: &str,
        return_as: &Argument,
    ) -> Result<ParsedArgumentResult, CommandParseError<'a>> {
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

        let mut url = try_url.ok_or_else(|| {
            CommandParseError::with_reply(
                "This command expects an image as an argument, but no image could be found."
                    .to_owned(),
                None,
            )
        })?;

        if url.starts_with("https://tenor.com/view/") {
            let page = self
                .reqwest_client
                .get(&url)
                .send()
                .await
                .map_err(|e| CommandParseError::with_reply(e.to_string(), None))?
                .text()
                .await
                .map_err(|e| CommandParseError::with_reply(e.to_string(), None))?;

            let gif_url = regexes::TENOR_GIF
                .find(&page)
                .and_then(|url| Some(url.as_str()))
                .ok_or_else(|| {
                    CommandParseError::with_reply(
                        "Failed to extract Tenor GIF URL".to_owned(),
                        None,
                    )
                })?;
            url = gif_url.to_owned();
        };

        match return_as {
            Argument::ImageBuffer => {
                let result = download_content(
                    &self.reqwest_client,
                    &url,
                    ABSOLUTE_INPUT_FILE_SIZE_LIMIT_BYTES,
                )
                .await
                .map_err(|e| CommandParseError::with_reply(e, None))?;

                let parsed_argument_result = match should_increment {
                    true => {
                        ParsedArgumentResult::increment(ParsedArgument::Binary(Bytes::from(result)))
                    }
                    false => ParsedArgumentResult::no_increment(ParsedArgument::Binary(
                        Bytes::from(result),
                    )),
                };
                Ok(parsed_argument_result)
            }
            Argument::ImageUrl => {
                let parsed_argument_result = match should_increment {
                    true => ParsedArgumentResult::increment(ParsedArgument::Text(url)),
                    false => ParsedArgumentResult::no_increment(ParsedArgument::Text(url)),
                };
                Ok(parsed_argument_result)
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
        let message_attachment_urls: Vec<Option<Cow<String>>> = messages
            .iter()
            .map(|message| {
                if let Some(embed) = message.embeds.first() {
                    if let Some(e) = &embed.url {
                        if e.starts_with("https://tenor.com/view/") {
                            return Some(Cow::Borrowed(e));
                        };
                    }
                    embed
                        .image
                        .as_ref()
                        .and_then(|img| Some(Cow::Borrowed(img.proxy_url.as_ref()?)))
                        .or_else(|| {
                            embed.thumbnail.as_ref().and_then(|thumbnail| {
                                Some(Cow::Borrowed(thumbnail.proxy_url.as_ref()?))
                            })
                        })
                        .or_else(|| {
                            embed.video.as_ref().and_then(|video| {
                                Some(Cow::Owned(format!(
                                    "{}?format=png",
                                    video.proxy_url.as_ref()?
                                )))
                            })
                        })
                } else {
                    Some(Cow::Borrowed(&message.attachments.first()?.proxy_url))
                }
            })
            .collect();

        message_attachment_urls
            .iter()
            .find(|attachment| attachment.is_some())?
            .as_ref()
            .and_then(|x| Some(x.to_string()))
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

    pub async fn get_average_processing_time(&self) -> f32 {
        self.metrics.read().await.processing.avg()
    }
}
