use crate::{
    badtranslator::BadTranslator,
    caching::{Ratelimits, Replies, Reply},
    command::context::Metrics,
    command::{
        command::{
            Argument, Command, CommandAvailability, CommandParseError, CommandParseErrorType,
            ParsedArgument, ParsedArgumentResult, ParsedCommand,
        },
        context::Context,
        registry::CommandRegistry,
    },
    config::Config,
    consts::{ABSOLUTE_INPUT_FILE_SIZE_LIMIT_BYTES, BOT_ID},
    database::Database,
    logging::Logger,
    metrics::GlobalMetrics,
    rest::patreon::Patron,
    util::{
        download_content, get_current_millis, get_guild_owner, get_sticker_url_from_message,
        regexes, Uptime,
    },
};

use bytes::Bytes;
use reqwest::Client as ReqwestClient;
use std::{
    borrow::{Borrow, Cow},
    sync::Arc,
    time::Instant,
};
use tokio::sync::RwLock;
use twilight_gateway::Cluster;
use twilight_http::Client as HttpClient;
use twilight_model::{channel::Message, guild::Permissions, id::UserId};

/// Takes an input string and prefix to parse and returns a tuple containing
/// the command invocation (such as `help`) and any arguments to the command
/// (such as `ping`).
fn get_command_and_args(content: &str, prefix: &str) -> Option<(String, Vec<String>)> {
    // if message doesnt start with prefix, or message is only the prefix, then ignore
    return if !content.starts_with(&prefix) || content.len() == prefix.len() {
        None
    } else {
        let raw = &content[prefix.len()..].trim_start();
        let raw_replaced = raw.replace("\n", " \n");
        let mut args = raw_replaced
            .split(' ')
            .map(|x| String::from(x))
            .collect::<Vec<String>>();
        let cmd = args[0].clone();
        args.remove(0);
        Some((cmd, args))
    };
}

/// Returns `Some(prefix)` if the prefix is the mention of the bot, otherwise `None`
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

/// The Assyst primary structure.
///
/// This structure contains all of the individual properties required for the bot to operate.
/// These properties are public because they are frequently used throughout the codebase.
///
/// Note that by default this instance has no connection to Discord. A connection
/// must be configured through a separately instantiated [`Cluster`] instance that then
/// may be attached to the [`Assyst`] instance using the [`Assyst::set_cluster`] method.
///
/// The `impl` for [`Assyst`] also contains all methods necessary for execution of a
/// command. This means that it contains the command and argument parser code, and also
/// has direct access to the [`Registry`] where commands are defined and called.
///
/// The key working goals of the Assyst codebase, and the bot itself, are the following:
///     - Minimal caching for optimized memory usage
///     - Responsive and distributed functionality
///     - High availability
///     - Unique feaures and design
///
/// The primary way Assyst achieves many of these fundamental goals are through the lack of
/// any sort of cache of raw Discord objects. This helps offload a lot of memory usage - in
/// fact, memory usage should never increase that much as the bot scales - and trades it off
/// as additional CPU time and bandwidth usage as things like users and guilds must always be
/// fetched from the Discord API instead of locally from a cache.
///
/// Because the bot itself is so lightweight, the entire thing should always be able to exist
/// in a single process. This is why there is no distributed functionality here. Instead,
/// distributed functionality exists in the first-party image processing API(s) that the
/// bot relies on, primarily WSI and Annmarie.
///
/// apina
pub struct Assyst {
    pub badtranslator: BadTranslator,
    pub cluster: Option<Cluster>,
    pub command_ratelimits: RwLock<Ratelimits>,
    pub config: Config,
    pub database: Database,
    pub http: HttpClient,
    pub logger: Logger,
    pub metrics: RwLock<GlobalMetrics>,
    pub patrons: RwLock<Vec<Patron>>,
    pub registry: CommandRegistry,
    pub replies: RwLock<Replies>,
    pub reqwest_client: ReqwestClient,
    pub started_at: u64,
}
impl Assyst {
    /// Create a new Assyst instance from a token. This method does NOT
    /// cause the bot to connect to Discord. The bot must interface with Discord
    /// using a separately instantiated [`Cluster`] instance and recieve events to
    /// process from that using the [`Assyst::handle_command`] method.
    ///
    /// Assyst itself is not configurable using this method.
    /// Assyst configuration exists in the config.toml file at the root
    /// of this project. Use that to configure the behaviour of the bot.
    pub async fn new(token: &str) -> Self {
        let http = HttpClient::new(token);
        let reqwest_client = ReqwestClient::new();
        let config = Config::new();
        let database = Database::new(2, config.database.to_url()).await.unwrap();
        let mut assyst = Assyst {
            badtranslator: BadTranslator::new(),
            cluster: None,
            command_ratelimits: RwLock::new(Ratelimits::new()),
            config,
            database,
            http,
            logger: Logger {},
            metrics: RwLock::new(GlobalMetrics::new()),
            patrons: RwLock::new(vec![]),
            registry: CommandRegistry::new(),
            replies: RwLock::new(Replies::new()),
            reqwest_client,
            started_at: get_current_millis(),
        };
        if assyst.config.disable_bad_translator {
            assyst.badtranslator.disable().await
        };
        assyst.registry.register_commands();
        assyst
    }

    /// Set the cluster instance that this instance of Assyst receives its events from.
    ///
    /// We can't do this in the constructor because it is impossible to have an initialized cluster
    /// when the instance of Assyst is constructed.
    pub fn set_cluster(&mut self, cluster: Cluster) {
        self.cluster = Some(cluster);
    }

    /// Handle an incoming message from Discord.
    ///
    /// This function takes a raw input [`Message`] object, parses it as a command (if valid)
    /// and executes the corresponding command, often with side effects like sending a message
    /// to the channel that the message was sent.
    ///
    /// The instance of [`Assyst`] in which this function is called must be [`Arc`]ed because
    /// it requires itself to be cloned during the execution process, since some actions
    /// happen on separate threads of execution.
    pub async fn handle_command(self: &Arc<Self>, _message: Message) -> Result<(), String> {
        // timing for use in ping command
        let start = Instant::now();

        let message = Arc::new(_message);

        // checking if user is blackisted from bot
        if self.config.user.blacklist.contains(&message.author.id.0) {
            return Ok(());
        }

        // parsing prefix from start of content
        let prefix;
        let mut prefix_is_mention = false;

        // selecting correct prefix based on the configuration
        // a.k.a prefix override, normal prefix, mention prefix?
        if self.config.prefix.r#override.len() == 0 {
            let mention_prefix = message_mention_prefix(&message.content);

            match mention_prefix {
                // if message starts with mention, thats the prefix
                Some(p) => {
                    prefix = Cow::Owned(p);
                    prefix_is_mention = true;
                }
                None => {
                    let try_prefix = self
                        .database
                        .get_or_set_prefix_for(
                            message.guild_id.unwrap().0,
                            &self.config.prefix.default,
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
            prefix = Cow::Borrowed(&self.config.prefix.r#override);
        };

        if !message.content.starts_with(prefix.borrow() as &str) {
            return Ok(());
        };

        // handling replies - set the source message for this command as the invocation
        // for this reply
        let mut replies = self.replies.write().await;
        let reply = replies
            .get_or_set_reply(Reply::new(message.clone()))
            .clone();

        let mut reply_lock = reply.lock().await;
        if reply_lock.has_expired() || reply_lock.in_use {
            return Ok(());
        };

        // we lock this specific command invocation.
        // new commands cannot be executed using this invocation while it is locked.

        // the logic here is that it will stop someone from editing the command message
        // to a new command while the original command is still being processed, which
        // should migitate issues with duplicate responses and ratelimit bypassing.
        reply_lock.in_use = true;
        drop(reply_lock);
        drop(replies);

        let metrics = Metrics {
            processing_time_start: start,
        };

        // display prefix is used in usage information
        // for help command and when a command has
        // invalid arguments and usage needs to be displayed
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

        // parsing and validating arguments for this command
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

        let command_instance = &self.registry.commands.get(command.calling_name).unwrap();

        // checking if the command is disabled
        let is_disabled = self
            .database
            .is_command_disabled(command_instance.name, message.guild_id.unwrap())
            .await;

        if is_disabled {
            let owner = get_guild_owner(&self.http, message.guild_id.unwrap())
                .await
                .map_err(|e| e.to_string())?;

            if owner != message.author.id
                && !self
                    .config
                    .user
                    .admins
                    .contains(&context.message.author.id.0)
            {
                return Ok(());
            };
        };

        let context_clone = context.clone();

        // checking if this command violates the ratelimits
        let mut ratelimit_lock = self.command_ratelimits.write().await;
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

        // run the command
        let command_result = self.registry.execute_command(command, context_clone).await;

        reply.lock().await.in_use = false;
        if let Err(err) = command_result {
            context
                .reply_err(&err.replace("\\n", "\n"))
                .await
                .map_err(|e| e.to_string())?;
        };

        self.logger
            .info(
                self.clone(),
                &format!("Command successfully executed: {}", command_instance.name),
            )
            .await;

        self.metrics
            .write()
            .await
            .processing
            .add(context.metrics.processing_time_start.elapsed().as_millis() as f32);

        Ok(())
    }

    /// Parse a command from a predefined [`Context`] and prefix.
    /// Returns Ok([`ParsedCommand`]) on a successful parse, and
    /// otherwise returns an Err with details about what failed to
    /// parse.
    ///
    /// Parse failures are expected because many input messages
    /// will not match the syntax of Assyst commands since users
    /// may not even be trying to run Assyst commands even if the
    /// message starts with the correct prefix.
    pub async fn parse_command(
        &self,
        context: &Arc<Context>,
        prefix: &str,
    ) -> Result<Option<ParsedCommand>, CommandParseError<'_>> {
        let content = &context.message.content;

        // extract the command name and arguments from the input message
        let command_details =
            get_command_and_args(&content, &prefix).unwrap_or(("".to_owned(), vec![]));
        let args_refs = command_details.1.iter().map(|x| &x[..]).collect::<Vec<_>>();

        // check if the command is actually valid
        let try_command = self
            .registry
            .get_command_from_name_or_alias(&command_details.0.to_ascii_lowercase());
        let command = match try_command {
            Some(c) => c,
            None => return Ok(None),
        };

        // check relevant permissions for the command
        match command.availability {
            CommandAvailability::Public => Ok(()), // everyone can run
            CommandAvailability::Private => {
                // bot admins can run (config-defined)
                if !self
                    .config
                    .user
                    .admins
                    .contains(&context.message.author.id.0)
                {
                    Err(CommandParseError::without_reply(
                        "Insufficient Permissions".to_owned(),
                        CommandParseErrorType::MissingPermissions,
                    ))
                } else {
                    Ok(())
                }
            }
            CommandAvailability::GuildOwner => {
                // guild owner *or* manage server *or* admin
                // get owner
                let owner = get_guild_owner(&self.http, context.message.guild_id.unwrap())
                    .await
                    .map_err(|_| CommandParseError::permission_validator_failed())?;

                // figure out permissions of the user through bitwise operations
                let member = self
                    .http
                    .guild_member(context.message.guild_id.unwrap(), context.message.author.id)
                    .await
                    .map_err(|_| CommandParseError::permission_validator_failed())?
                    .unwrap();

                let roles = self
                    .http
                    .roles(context.message.guild_id.unwrap())
                    .await
                    .map_err(|_| CommandParseError::permission_validator_failed())?;

                let member_roles = roles
                    .iter()
                    .filter(|r| member.roles.contains(&r.id))
                    .collect::<Vec<_>>();
                let member_permissions =
                    member_roles.iter().fold(0, |a, r| a | r.permissions.bits());
                let member_is_manager = member_permissions & Permissions::ADMINISTRATOR.bits()
                    == Permissions::ADMINISTRATOR.bits()
                    || member_permissions & Permissions::MANAGE_GUILD.bits()
                        == Permissions::MANAGE_GUILD.bits();

                if owner == context.message.author.id
                    || self
                        .config
                        .user
                        .admins
                        .contains(&context.message.author.id.0)
                    || member_is_manager
                {
                    Ok(())
                } else {
                    Err(CommandParseError::with_reply(
                        "You need the manage server permission to use this command.".to_owned(),
                        None,
                        CommandParseErrorType::MissingPermissions,
                    ))
                }
            }
        }?;

        let parsed_args = self.parse_arguments(context, &command, args_refs).await?;
        Ok(Some(ParsedCommand {
            calling_name: &command.name,
            args: parsed_args,
        }))
    }

    /// Parses arguments from a context and a set of predefined, expected 'argument types'.
    /// Returns Ok with the parsed arguments on success and an Err with what failed to parse
    /// in the event of a failure.
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

    /// Parses an individual argument.
    ///
    /// Looks at the type of the argument and what is being parsed into that type.
    /// If possible, this method will do that conversion. If not, the command has
    /// invalid syntax and this function will return an Err.
    async fn parse_argument<'a>(
        &self,
        context: &Arc<Context>,
        command: &'a Command,
        args: &Vec<&str>,
        arg: &Argument,
        index: &usize,
    ) -> Result<ParsedArgumentResult, CommandParseError<'a>> {
        // check the next type of argument and parse as appropriate
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
                // we need this 'nonoptional' function because
                // standalone rust doesn't support async recursion. sad!
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

    // this function only exists because rust doesn't support
    // async recursion without the use of a dependency.
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

    /// Parses an image from the command for use with image manipulation commands.
    async fn parse_image_argument<'a>(
        &self,
        message: &Message,
        argument: &str,
        return_as: &Argument,
    ) -> Result<ParsedArgumentResult, CommandParseError<'a>> {
        // TODO: rework this to be less hacky? maybe have a proper
        // defined priority somewhere and use that instead of trying everything
        // until it works. changing priorities right now is fiddly at best
        let mut should_increment = true;
        let mut try_url = self.validate_user_argument(argument).await;
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
            try_url = self.validate_message_reply_attachment(message);
            if try_url.is_some() {
                should_increment = false
            };
        }
        if try_url.is_none() {
            try_url = self.validate_emoji_argument(argument).await;
        }
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

        // tenor urls only typically return a png, so this code visits the url
        // and extracts the appropriate GIF url from the page.
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

    /// Looks at an input string and checks if it is a valid Unicode or custom emoji.
    async fn validate_emoji_argument(&self, argument: &str) -> Option<String> {
        let unicode_emoji = emoji::lookup_by_glyph::lookup(argument);
        if let Some(e) = unicode_emoji {
            let codepoint = e
                .codepoint
                .to_lowercase()
                .replace(" ", "-")
                .replace("-fe0f", "");
            let emoji_url = format!("https://derpystuff.gitlab.io/webstorage3/container/twemoji-JedKxRr7RNYrgV9Sauy8EGAu/{}.png", codepoint);
            return Some(emoji_url);
        }

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

    /// Looks at a source [`Message`] and see if it has any attachments, returning the
    /// URL to first one if it does.
    fn validate_message_attachment(&self, message: &Message) -> Option<String> {
        message
            .attachments
            .first()
            .and_then(|a| Some(a.url.clone()))
            .or_else(|| get_sticker_url_from_message(message))
    }

    /// Looks at a source [`Message`] and see if it has any embeds with an image, 
    /// returning the first one if it does.
    fn validate_message_embed<'a>(&self, message: &'a Message) -> Option<Cow<'a, String>> {
        let embed = message.embeds.first()?;

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
    }

    /// If the command invocation is replying to the message, check if the message being replied
    /// to has an attachment
    fn validate_message_reply_attachment(&self, message: &Message) -> Option<String> {
        let reply = message.referenced_message.as_ref()?;
        let attachment = self.validate_message_attachment(reply);
        if attachment.is_some() {
            return attachment;
        };
        let embed = self.validate_message_embed(reply)?;
        Some(embed.to_string())
    }

    /// Load the last N messages in the channel where the command was invocated,
    /// and check if any of them have an attachment or embed
    async fn validate_previous_message_attachment(&self, message: &Message) -> Option<String> {
        let messages = self.http.channel_messages(message.channel_id).await.ok()?;
        let message_attachment_urls: Vec<Option<Cow<String>>> = messages
            .iter()
            .map(|message| {
                if let Some(_) = message.embeds.first() {
                    self.validate_message_embed(message)
                } else {
                    message
                        .attachments
                        .first()
                        .and_then(|a| Some(Cow::Borrowed(&a.url)))
                        .or_else(|| Some(Cow::Owned(get_sticker_url_from_message(message)?)))
                }
            })
            .collect();

        message_attachment_urls
            .iter()
            .find(|attachment| attachment.is_some())?
            .as_ref()
            .and_then(|x| Some(x.to_string()))
    }

    /// Check if the command invocation contains a valid URL
    fn validate_url_argument(&self, argument: &str) -> Option<String> {
        if regexes::URL.is_match(argument) {
            Some(argument.to_owned())
        } else {
            None
        }
    }

    /// Check if the command invocation contains a valid user mention
    /// or ID, and use the avatar of that user if it does
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

    /// Get the average time to process commands in ms
    pub async fn get_average_processing_time(&self) -> f32 {
        self.metrics.read().await.processing.avg()
    }

    /// Get the [`Uptime`] of this instance of Assyst
    pub fn uptime(&self) -> Uptime {
        Uptime::new(get_current_millis() - self.started_at)
    }
}
