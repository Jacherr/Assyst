use crate::{
    badtranslator::BadTranslator,
    caching::Ratelimits,
    command::command::CommandAvailability,
    util::{get_current_millis, Uptime},
};
use crate::{
    caching::{Replies, Reply},
    command::context::Metrics,
    database::Database,
};
use crate::{
    command::command::{
        Argument, Command, CommandParseError, CommandParseErrorType, ParsedArgument,
        ParsedArgumentResult, ParsedCommand,
    },
    consts::BOT_ID,
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
    pub annmarie_url: Box<str>,
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

fn get_command_and_args<'a>(content: &'a str, prefix: &str) -> Option<(&'a str, Vec<&'a str>)> {
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

fn message_mention_prefix(content: &str) -> Option<String> {
    let mention_no_nickname = format!("<@{}>", BOT_ID);
    let mention_nickname = format!("<@!{}>", BOT_ID);

    if content.starts_with(&mention_no_nickname) {
        Some(mention_no_nickname)
    } else if content.starts_with(&mention_nickname) {
        Some(mention_nickname)
    } else {
        None
    }
}

pub struct Assyst {
    pub command_ratelimits: RwLock<Ratelimits>,
    pub config: Config,
    pub database: Database,
    pub started_at: u64,
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
            command_ratelimits: RwLock::new(Ratelimits::new()),
            config,
            database,
            started_at: get_current_millis(),
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
        let mut prefix_is_mention = false;
        if self.config.prefix_override.len() == 0 {
            let mention_prefix = message_mention_prefix(&message.content);

            match mention_prefix {
                Some(p) => {
                    prefix = Cow::Owned(p);
                    prefix_is_mention = true;
                }
                None => {
                    let try_prefix = self
                        .database
                        .get_or_set_prefix_for(
                            message.guild_id.unwrap().0,
                            &self.config.default_prefix,
                        )
                        .await
                        .map_err(|err| err.to_string())?;

                    match try_prefix {
                        Some(p) => {
                            prefix = p;
                        }
                        None => return Ok(()),
                    };
                }
            }
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

        let metrics = Metrics {
            processing_time_start: start,
        };

        let display_prefix = if prefix_is_mention {
            "@Assyst "
        } else {
            prefix.borrow()
        };

        let context = Arc::new(Context::new(
            self.clone(),
            message.clone(),
            metrics,
            String::from(display_prefix.clone()),
            reply.clone(),
        ));

        let t_command = self.parse_command(&context, &prefix).await;

        let command = match t_command {
            Ok(res) => match res {
                Some(c) => c,
                None => {
                    reply.lock().await.in_use = false;
                    return Ok(());
                }
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
                reply.lock().await.in_use = false;
                return Ok(());
            }
        };

        let context_clone = context.clone();

        let mut ratelimit_lock = self.command_ratelimits.write().await;
        let command_instance = &self.registry.commands.get(command.calling_name).unwrap();
        let command_ratelimit = ratelimit_lock
            .time_until_guild_command_usable(message.guild_id.unwrap(), &command_instance.name);
        match command_ratelimit {
            Some(r) => {
                reply.lock().await.in_use = false;
                context
                    .reply_err(&format!(
                        "This command is on cooldown for {:.2} seconds.",
                        (r as f64 / 1000f64)
                    ))
                    .await
                    .map_err(|e| e.to_string())?;
                return Ok(());
            }
            None => {}
        };
        ratelimit_lock.set_command_expire_at(message.guild_id.unwrap(), &command_instance);
        drop(ratelimit_lock);

        let command_result = self.registry.execute_command(command, context_clone).await;

        reply.lock().await.in_use = false;
        if let Err(err) = command_result {
            context
                .reply_err(&err.replace("\\n", "\n"))
                .await
                .map_err(|e| e.to_string())?;
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
        context: &Arc<Context>,
        prefix: &str,
    ) -> Result<Option<ParsedCommand>, CommandParseError<'_>> {
        let content = &context.message.content;
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
                if !self.config.admins.contains(&context.message.author.id.0) {
                    Err(CommandParseError::without_reply(
                        "Insufficient Permissions".to_owned(),
                        CommandParseErrorType::MissingPermissions,
                    ))
                } else {
                    Ok(())
                }
            }
            CommandAvailability::GuildOwner => {
                let guild = self
                    .http
                    .guild(context.message.guild_id.unwrap())
                    .await
                    .map_err(|e| {
                        CommandParseError::with_reply(
                            e.to_string(),
                            None,
                            CommandParseErrorType::MediaDownloadFail,
                        )
                    })?
                    .ok_or_else(|| {
                        CommandParseError::with_reply(
                            "Permission validator failed".to_owned(),
                            None,
                            CommandParseErrorType::MissingPermissions,
                        )
                    })?;

                if guild.owner_id == context.message.author.id
                    || self.config.admins.contains(&context.message.author.id.0)
                {
                    Ok(())
                } else {
                    Err(CommandParseError::without_reply(
                        "Insufficient Permissions".to_owned(),
                        CommandParseErrorType::MissingPermissions,
                    ))
                }
            }
        }?;

        let parsed_args = self
            .parse_arguments(context, &command, command_details.1)
            .await?;
        Ok(Some(ParsedCommand {
            calling_name: &command.name,
            args: parsed_args,
        }))
    }

    async fn parse_arguments<'a>(
        &self,
        context: &Arc<Context>,
        command: &'a Command,
        args: Vec<&str>,
    ) -> Result<Vec<ParsedArgument>, CommandParseError<'a>> {
        let mut parsed_args: Vec<ParsedArgument> = vec![];
        let mut index: usize = 0;
        for arg in &command.args {
            let result = self
                .parse_argument(context, command, &args, arg, &index)
                .await?;
            parsed_args.push(result.value);
            if result.should_break {
                break;
            } else if result.should_increment_index {
                index += 1
            };
        }
        Ok(parsed_args)
    }

    async fn parse_argument<'a>(
        &self,
        context: &Arc<Context>,
        command: &'a Command,
        args: &Vec<&str>,
        arg: &Argument,
        index: &usize,
    ) -> Result<ParsedArgumentResult, CommandParseError<'a>> {
        match arg {
            Argument::Integer | Argument::Decimal => {
                if args.len() <= *index {
                    return Err(CommandParseError::with_reply(
                        "This command expects a numerical argument, but no argument was provided."
                            .to_owned(),
                        Some(command),
                        CommandParseErrorType::MissingArgument,
                    ));
                }

                let float = args[*index].parse::<f64>().map_err(|_| {
                    CommandParseError::with_reply(
                        format!("Invalid number provided: {}", args[*index]),
                        Some(command),
                        CommandParseErrorType::MissingArgument,
                    )
                })?;

                return match arg {
                    Argument::Decimal => Ok(ParsedArgumentResult::increment(ParsedArgument::Text(
                        float.to_string(),
                    ))),

                    Argument::Integer => Ok(ParsedArgumentResult::increment(ParsedArgument::Text(
                        format!("{:.0}", float),
                    ))),

                    _ => unreachable!(),
                };
            }

            Argument::Choice(choices) => {
                if args.len() <= *index {
                    return Err(CommandParseError::with_reply(
                        format!("This command expects a choice argument (one of {:?}), but no argument was provided.", choices),
                            Some(command),
                            CommandParseErrorType::MissingArgument
                    ));
                }

                let choice = match choices.iter().find(|&&choice| choice == args[*index]) {
                    Some(k) => k,
                    None => {
                        return Err(CommandParseError::with_reply(
                            format!("Cannot find given argument in {:?}", choices),
                            Some(command),
                            CommandParseErrorType::InvalidArgument,
                        ))
                    }
                };

                Ok(ParsedArgumentResult::increment(ParsedArgument::Choice(
                    choice,
                )))
            }

            Argument::ImageUrl | Argument::ImageBuffer => {
                let argument_to_pass = if args.len() <= *index {
                    ""
                } else {
                    args[*index]
                };
                self.parse_image_argument(&context.message, argument_to_pass, arg)
                    .await
            }

            Argument::String => {
                if args.len() <= *index {
                    return Err(CommandParseError::with_reply(
                        "This command expects a text argument that was not provided.".to_owned(),
                        Some(command),
                        CommandParseErrorType::MissingArgument,
                    ));
                }
                Ok(ParsedArgumentResult::increment(ParsedArgument::Text(
                    args[*index].to_owned(),
                )))
            }

            Argument::StringRemaining => {
                if args.len() <= *index {
                    return Err(CommandParseError::with_reply(
                        "This command expects a text argument that was not provided.".to_owned(),
                        Some(command),
                        CommandParseErrorType::MissingArgument,
                    ));
                }
                Ok(ParsedArgumentResult::r#break(ParsedArgument::Text(
                    args[*index..].join(" "),
                )))
            }

            Argument::Optional(a) => {
                let result = self
                    .parse_argument_nonoptional(&context.message, command, args, &**a, index)
                    .await;
                match result {
                    Ok(p) => Ok(p),
                    Err(e) => {
                        match e.error_type {
                            CommandParseErrorType::MissingArgument => {
                                return Ok(ParsedArgumentResult::increment(ParsedArgument::Nothing))
                            }
                            _ => (),
                        }
                        Err(e)
                    }
                }
            }

            Argument::OptionalWithDefault(a, d) => {
                let result = self
                    .parse_argument_nonoptional(&context.message, command, args, &**a, index)
                    .await;
                match result {
                    Ok(p) => Ok(p),
                    Err(e) => {
                        match e.error_type {
                            CommandParseErrorType::MissingArgument => {
                                return Ok(ParsedArgumentResult::increment(ParsedArgument::Text(
                                    d.to_owned().to_owned(),
                                )))
                            }
                            _ => (),
                        }
                        Err(e)
                    }
                }
            }

            Argument::OptionalWithDefaultDynamic(arg, default) => {
                let result = self
                    .parse_argument_nonoptional(&context.message, command, args, &**arg, index)
                    .await;

                match result {
                    Ok(p) => Ok(p),
                    Err(e) => {
                        match e.error_type {
                            CommandParseErrorType::MissingArgument => {
                                return Ok(ParsedArgumentResult::increment(default(
                                    context.clone(),
                                )))
                            }
                            _ => (),
                        }
                        Err(e)
                    }
                }
            }
        }
    }

    async fn parse_argument_nonoptional<'a>(
        &self,
        message: &Message,
        command: &'a Command,
        args: &Vec<&str>,
        arg: &Argument,
        index: &usize,
    ) -> Result<ParsedArgumentResult, CommandParseError<'a>> {
        match arg {
            Argument::Integer | Argument::Decimal => {
                if args.len() <= *index {
                    return Err(CommandParseError::with_reply(
                        "This command expects a numerical argument, but no argument was provided."
                            .to_owned(),
                        Some(command),
                        CommandParseErrorType::MissingArgument,
                    ));
                }

                let float = args[*index].parse::<f64>().map_err(|_| {
                    CommandParseError::with_reply(
                        format!("Invalid number provided: {}", args[*index]),
                        Some(command),
                        CommandParseErrorType::InvalidArgument,
                    )
                })?;

                return match arg {
                    Argument::Decimal => Ok(ParsedArgumentResult::increment(ParsedArgument::Text(
                        float.to_string(),
                    ))),

                    Argument::Integer => Ok(ParsedArgumentResult::increment(ParsedArgument::Text(
                        format!("{:.0}", float),
                    ))),

                    _ => unreachable!(),
                };
            }

            Argument::Choice(choices) => {
                if args.len() <= *index {
                    return Err(CommandParseError::with_reply(
                        format!("This command expects a choice argument (one of {:?}), but no argument was provided.", choices),
                            Some(command),
                            CommandParseErrorType::MissingArgument
                    ));
                }

                let choice = match choices.iter().find(|&&choice| choice == args[*index]) {
                    Some(k) => k,
                    None => {
                        return Err(CommandParseError::with_reply(
                            format!("Cannot find given argument in {:?}", choices),
                            Some(command),
                            CommandParseErrorType::InvalidArgument,
                        ))
                    }
                };

                Ok(ParsedArgumentResult::increment(ParsedArgument::Choice(
                    choice,
                )))
            }
            Argument::ImageUrl | Argument::ImageBuffer => {
                let argument_to_pass = if args.len() <= *index {
                    ""
                } else {
                    args[*index]
                };
                self.parse_image_argument(message, argument_to_pass, arg)
                    .await
            }
            Argument::String => {
                if args.len() <= *index {
                    return Err(CommandParseError::with_reply(
                        "This command expects a text argument that was not provided.".to_owned(),
                        Some(command),
                        CommandParseErrorType::MissingArgument,
                    ));
                }
                Ok(ParsedArgumentResult::increment(ParsedArgument::Text(
                    args[*index].to_owned(),
                )))
            }
            Argument::StringRemaining => {
                if args.len() <= *index {
                    return Err(CommandParseError::with_reply(
                        "This command expects a text argument that was not provided.".to_owned(),
                        Some(command),
                        CommandParseErrorType::MissingArgument,
                    ));
                }
                Ok(ParsedArgumentResult::r#break(ParsedArgument::Text(
                    args[*index..].join(" "),
                )))
            }
            _ => unreachable!(),
        }
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
                CommandParseErrorType::MissingArgument,
            )
        })?;

        if url.starts_with("https://tenor.com/view/") {
            let page = self
                .reqwest_client
                .get(&url)
                .send()
                .await
                .map_err(|e| {
                    CommandParseError::with_reply(
                        e.to_string(),
                        None,
                        CommandParseErrorType::MediaDownloadFail,
                    )
                })?
                .text()
                .await
                .map_err(|e| {
                    CommandParseError::with_reply(
                        e.to_string(),
                        None,
                        CommandParseErrorType::MediaDownloadFail,
                    )
                })?;

            let gif_url = regexes::TENOR_GIF
                .find(&page)
                .and_then(|url| Some(url.as_str()))
                .ok_or_else(|| {
                    CommandParseError::with_reply(
                        "Failed to extract Tenor GIF URL".to_owned(),
                        None,
                        CommandParseErrorType::MediaDownloadFail,
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
                .map_err(|e| {
                    CommandParseError::with_reply(e, None, CommandParseErrorType::MediaDownloadFail)
                })?;

                if result.len() == 0 {
                    return Err(CommandParseError::with_reply(
                        "The media download failed because no data was received.".to_owned(),
                        None,
                        CommandParseErrorType::MediaDownloadFail,
                    ));
                }

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
            .and_then(|emoji_id_capture| emoji_id_capture.get(2))
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
        Some(message.attachments.first()?.url.clone())
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
                        .and_then(|img| Some(Cow::Borrowed(img.url.as_ref()?)))
                        .or_else(|| {
                            embed
                                .thumbnail
                                .as_ref()
                                .and_then(|thumbnail| Some(Cow::Borrowed(thumbnail.url.as_ref()?)))
                        })
                        .or_else(|| {
                            embed.video.as_ref().and_then(|video| {
                                Some(Cow::Owned(format!("{}", video.url.as_ref()?)))
                            })
                        })
                } else {
                    message
                        .attachments
                        .first()
                        .and_then(|a| Some(Cow::Borrowed(&a.url)))
                        .or_else(|| {
                            message.stickers.get(0).and_then(|s| {
                                Some(Cow::Owned(format!(
                                    "https://distok.top/stickers/{}/{}.gif",
                                    s.pack_id, s.id
                                )))
                            })
                        })
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

    pub fn uptime(&self) -> Uptime {
        Uptime::new(get_current_millis() - self.started_at)
    }
}
