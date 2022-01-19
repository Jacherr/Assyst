use crate::{
    badtranslator::BadTranslator,
    caching::{Ratelimits, Replies, Reply},
    command::{
        command::{
            Argument, Command, CommandAvailability, CommandParseError, CommandParseErrorType,
            FlagKind, ParsedArgument, ParsedArgumentResult, ParsedCommand, ParsedFlagKind,
        },
        context::{Context, Metrics},
        parse,
        registry::CommandRegistry,
    },
    logger,
    metrics::GlobalMetrics,
    rest::{patreon::Patron, wsi::wsi_listen},
    util::{get_current_millis, get_guild_owner, is_guild_manager, regexes, Uptime},
};

use assyst_common::{config::Config, consts::BOT_ID};
use assyst_database::Database;
use async_recursion::async_recursion;
use regex::Captures;
use reqwest::Client as ReqwestClient;
use shared::{job::JobResult, fifo::FifoSend};
use std::{
    borrow::{Borrow, Cow},
    collections::{HashSet, HashMap},
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::Instant,
};
use tokio::sync::{Mutex, RwLock, mpsc::UnboundedSender, oneshot::Sender};
use twilight_http::Client as HttpClient;
use twilight_model::channel::{Channel, GuildChannel, Message};

fn get_command(content: &str, prefix: &str) -> Option<String> {
    get_raw_args(content, prefix, 0)
        .map(|x| x.into_iter())
        .and_then(|mut x| x.next())
}

fn get_raw_args(content: &str, prefix: &str, skip: usize) -> Option<Vec<String>> {
    if !content.starts_with(&prefix) || content.len() == prefix.len() {
        return None;
    }

    let raw = &content[prefix.len()..].trim();
    let raw_replaced = raw.replace("\n", " \n");
    Some(
        raw_replaced
            .split(' ')
            .skip(skip)
            .map(|x| String::from(x))
            .collect::<Vec<String>>(),
    )
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
    pub guilds: Mutex<HashSet<u32>>,
    pub badtranslator: BadTranslator,
    pub command_ratelimits: RwLock<Ratelimits>,
    pub config: Arc<Config>,
    pub database: Arc<Database>,
    pub http: HttpClient,
    pub metrics: GlobalMetrics,
    pub patrons: RwLock<Vec<Patron>>,
    pub registry: CommandRegistry,
    pub replies: RwLock<Replies>,
    pub reqwest_client: ReqwestClient,
    pub started_at: u64,
    pub commands_executed: AtomicU64,
    wsi_tx: UnboundedSender<(Sender<JobResult>, FifoSend, usize)>
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
        let config = Arc::new(Config::new());
        let database = Database::new(2, config.database.to_url())
            .await
            .map(Arc::new)
            .unwrap();
        
        let config_clone = config.clone();
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<(Sender<JobResult>, FifoSend, usize)>();

        let mut assyst = Assyst {
            guilds: Mutex::new(HashSet::new()),
            badtranslator: BadTranslator::new(),
            command_ratelimits: RwLock::new(Ratelimits::new()),
            config,
            database,
            http,
            metrics: GlobalMetrics::new().expect("Failed to create metric registry"),
            patrons: RwLock::new(vec![]),
            registry: CommandRegistry::new(),
            replies: RwLock::new(Replies::new()),
            reqwest_client,
            started_at: get_current_millis(),
            commands_executed: AtomicU64::new(0),
            wsi_tx: tx
        };
        if assyst.config.disable_bad_translator {
            assyst.badtranslator.disable().await
        }

        tokio::spawn(async move {
            wsi_listen(rx,&config_clone.url.wsi.to_string()).await;
        });

        assyst.registry.register_commands();
        assyst
    }

    pub fn send_to_wsi(&self, sender: Sender<JobResult>, job: FifoSend, premium_level: usize) {
        self.wsi_tx.send((sender, job, premium_level)).unwrap();
    }

    /// Add a guild to cached guild list
    pub async fn add_guild_to_list(&self, guild: u64) -> bool {
        self.guilds.lock().await.insert((guild >> 22) as u32)
    }

    /// Checks if guild is in cached guild list
    pub async fn guild_in_list(&self, guild: u64) -> bool {
        self.guilds.lock().await.contains(&((guild >> 22) as u32))
    }

    /// Remove a guild from cached guild list
    pub async fn remove_guild_from_list(&self, guild: u64) {
        self.guilds.lock().await.remove(&((guild >> 22) as u32));
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
    pub async fn handle_command(
        self: &Arc<Self>,
        _message: Message,
        from_update: bool,
    ) -> Result<(), String> {
        // timing for use in ping command
        let start = Instant::now();

        let message = Arc::new(_message);

        if from_update && message.edited_timestamp.is_none() {
            return Ok(());
        }

        // checking if user is blackisted from bot
        if self.config.user.blacklist.contains(&message.author.id.0) {
            return Ok(());
        }

        // parsing prefix from start of content
        let prefix;
        let mut prefix_is_mention = false;

        // selecting correct prefix based on the configuration
        // a.k.a prefix override, normal prefix, mention prefix?
        if self.config.prefix.r#override.is_empty() {
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

        if !message.content.starts_with(prefix.as_ref()) {
            return Ok(());
        }

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
            String::from(display_prefix),
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
                        Some(c) => format!(
                            "{}\nUsage: {}{} {}",
                            e.error, prefix, c.name, c.metadata.usage
                        ),
                        None => e.error,
                    };

                    context.reply_err(err).await.map_err(|e| e.to_string())?;
                }
                reply.lock().await.in_use = false;
                return Ok(());
            }
        };

        let command_instance = self.registry.commands.get(command.calling_name).unwrap();

        // checking if the command is disabled
        let is_guild_disabled = self
            .database
            .is_command_disabled(command_instance.name, message.guild_id.unwrap())
            .await;

        if is_guild_disabled {
            let owner = get_guild_owner(&self.http, message.guild_id.unwrap())
                .await
                .map_err(|e| e.to_string())?;

            if owner != message.author.id && !self.user_is_admin(context.author_id().0) {
                return Ok(());
            };
        };

        if command_instance.nsfw {
            let channel = self
                .http
                .channel(message.channel_id)
                .await
                .map_err(|e| format!("fetching channel for nsfw check fail: {}", e.to_string()))?
                .unwrap();

            if let Channel::Guild(guild) = channel {
                if let GuildChannel::Text(guild_text) = guild {
                    if !guild_text.nsfw {
                        context
                            .reply_err("This command is limited to NSFW text channels only.")
                            .await
                            .map_err(|e| e.to_string())?;
                        return Ok(());
                    }
                } else {
                    context
                        .reply_err("This command is limited to NSFW text channels only.")
                        .await
                        .map_err(|e| e.to_string())?;
                    return Ok(());
                }
            } else {
                unreachable!()
            }
        }

        let is_global_disabled = command_instance.disabled;

        if is_global_disabled && !self.user_is_admin(context.author_id().0) {
            context
                .reply_err("This command is globally disabled. :(")
                .await
                .map_err(|e| e.to_string())?;
            return Ok(());
        }

        let context_clone = context.clone();

        // checking if this command violates the ratelimits
        let mut ratelimit_lock = self.command_ratelimits.write().await;
        let command_ratelimit = ratelimit_lock
            .time_until_guild_command_usable(message.guild_id.unwrap(), &command_instance.name);

        match command_ratelimit {
            Some(r) => {
                reply.lock().await.in_use = false;
                context
                    .reply_err(format!(
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

        self.metrics.add_command();
        // run the command
        let command_result = self.registry.execute_command(command, context_clone).await;

        self.metrics.delete_command();

        reply.lock().await.in_use = false;
        if let Err(err) = command_result {
            let err_dsc = err.to_string().replace("\\n", "\n");

            context
                .reply_err(err_dsc)
                .await
                .map_err(|e| e.to_string())?;
        };

        self.metrics
            .add_processing_time(context.metrics.processing_time_start.elapsed().as_millis() as f64);

        self.commands_executed.fetch_add(1, Ordering::Relaxed);

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
        // extract the command name
        let mut command = get_command(&context.message.content, prefix).unwrap_or_else(String::new);
        command.make_ascii_lowercase();

        // check if the command is actually valid
        let try_command = self.registry.get_command_from_name_or_alias(&command);

        let command = match try_command {
            Some(c) => c,
            None => return Ok(None),
        };

        // clone context and message so the flag parser can mutate the message content
        // this is required because we don't want to have flags in the arguments
        let mut context = Context::clone(&context);
        let mut message = Message::clone(&context.message);

        let flags = {
            let (content, flags) = self.parse_flag(&message.content, command);
            let content = String::from(content);
            message.content = content;
            context.message = Arc::new(message);
            flags
        };

        // shadow old context variable
        let context = Arc::new(context);

        // get all other arguments from the fake context we've just created
        let args = get_raw_args(&context.message.content, prefix, 1).unwrap_or_else(Vec::new);

        let args_refs = args.iter().map(|x| x.as_str()).collect::<Vec<&str>>();

        // check relevant permissions for the command
        match command.availability {
            CommandAvailability::Public => Ok(()), // everyone can run
            CommandAvailability::Private => {
                // bot admins can run (config-defined)
                if !self.user_is_admin(context.author_id().0) {
                    Err(CommandParseError::without_reply(
                        "Insufficient Permissions".to_owned(),
                        CommandParseErrorType::MissingPermissions,
                    ))
                } else {
                    Ok(())
                }
            }
            CommandAvailability::GuildOwner => {
                let is_manager = is_guild_manager(
                    &self.http,
                    context.message.guild_id.unwrap(),
                    context.message.author.id,
                )
                .await
                .map_err(|_| CommandParseError::permission_validator_failed())?;

                let is_bot_admin = self.user_is_admin(context.author_id().0);

                if is_manager || is_bot_admin {
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

        let parsed_args = self.parse_arguments(&context, &command, args_refs).await?;
        Ok(Some(ParsedCommand {
            calling_name: &command.name,
            args: parsed_args,
            flags,
        }))
    }

    fn parse_flag<'a, 'b>(
        &self,
        content: &'a str,
        command: &'b Command,
    ) -> (Cow<'a, str>, HashMap<&'b str, Option<ParsedFlagKind>>) {
        let mut flags = HashMap::new();

        let new_content = regexes::COMMAND_FLAG.replace_all(content, |captures: &Captures| {
            // capture group @ index 2 is for flag values with surrounding quotes
            let has_quotes = captures.get(2).is_some();

            let mut iter = captures.iter().skip(1);

            // unwraps are safe - regex wouldn't match if these were None
            let name = iter.next().flatten().unwrap().as_str();
            let value = iter
                .next()
                .unwrap()
                .or_else(|| iter.next().flatten())
                .map(|x| x.as_str());

            let (name, kind) = match command.flags.get(name) {
                Some(c) => c,
                None => {
                    // if the flag doesn't exist, we need to "reconstruct" the original matched string
                    return format!(" -{}{}", name, {
                        if has_quotes {
                            // if it has quotes, we need to add them back
                            format!(" \"{}\"", value.unwrap_or(""))
                        } else {
                            value.map(|x| format!(" {}", x)).unwrap_or_else(String::new)
                        }
                    });
                }
            };

            let parsed_value = match kind {
                None => None,
                Some(FlagKind::Text) => value.map(ToOwned::to_owned).map(ParsedFlagKind::Text),
                Some(FlagKind::Boolean) => value
                    .and_then(|x| x.parse::<bool>().ok())
                    .map(ParsedFlagKind::Boolean),
                Some(FlagKind::Number) => value
                    .and_then(|x| x.parse::<u64>().ok())
                    .map(ParsedFlagKind::Number),
                Some(FlagKind::Decimal) => value
                    .and_then(|x| x.parse::<f64>().ok())
                    .map(ParsedFlagKind::Decimal),
                Some(FlagKind::Choice(choices)) => value
                    .and_then(|v| choices.iter().find(|&&x| x == v))
                    .copied()
                    .map(ToOwned::to_owned)
                    .map(ParsedFlagKind::Text),
                Some(FlagKind::List) => value
                    .map(|v| v.split(' ').map(ToOwned::to_owned).collect::<Vec<_>>())
                    .map(ParsedFlagKind::List),
            };

            flags.insert(*name, parsed_value);

            String::new()
        });

        (new_content, flags)
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
    #[async_recursion]
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
                return parse::argument_type::numerical(args, arg, command, *index);
            }

            Argument::Choice(choices) => {
                return parse::argument_type::choice(choices, args, command, *index);
            }

            Argument::ImageUrl | Argument::ImageBuffer => {
                let argument_to_pass = if args.len() <= *index {
                    ""
                } else {
                    args[*index]
                };
                parse::subsections::parse_image_argument(
                    context,
                    &context.message,
                    argument_to_pass,
                    arg,
                )
                .await
            }

            Argument::String => {
                return parse::argument_type::string(args, command, *index);
            }

            Argument::StringRemaining => {
                return parse::argument_type::string_remaining(context, args, command, *index);
            }

            Argument::Optional(a)
            | Argument::OptionalWithDefault(a, _)
            | Argument::OptionalWithDefaultDynamic(a, _) => {
                let result = self
                    .parse_argument(context, command, args, &**a, index)
                    .await;

                match result {
                    Ok(p) => Ok(p),
                    Err(e) => match e.error_type {
                        CommandParseErrorType::MissingArgument => match arg {
                            Argument::Optional(_) => {
                                Ok(ParsedArgumentResult::increment(ParsedArgument::Nothing))
                            }

                            Argument::OptionalWithDefault(_, d) => {
                                Ok(ParsedArgumentResult::increment(ParsedArgument::Text(
                                    d.to_owned().to_owned(),
                                )))
                            }

                            Argument::OptionalWithDefaultDynamic(_, default) => {
                                Ok(ParsedArgumentResult::increment(default(context.clone())))
                            }

                            _ => unreachable!(),
                        },
                        _ => Err(e),
                    },
                }
            }
        }
    }

    /// Check if the command invocation contains a valid URL
    pub fn validate_url_argument(&self, argument: &str) -> Option<String> {
        if regexes::URL.is_match(argument) {
            Some(argument.to_owned())
        } else {
            None
        }
    }

    pub async fn initialize_bt(&self) {
        match self.database.get_bt_channels().await {
            Ok(channels) => self.badtranslator.set_channels(channels).await,
            Err(e) => {
                logger::fatal(
                    self,
                    &format!(
                        "Fetching BadTranslator channels failed, disabling feature... {:?}",
                        e
                    ),
                )
                .await;
                self.badtranslator.disable().await;
            }
        }
    }

    /// Get the average time to process commands in ms
    pub async fn get_average_processing_time(&self) -> f32 {
        self.metrics.avg_processing_time()
    }

    /// Get the [`Uptime`] of this instance of Assyst
    pub fn uptime(&self) -> Uptime {
        Uptime::new(get_current_millis() - self.started_at)
    }

    pub fn user_is_admin(&self, id: u64) -> bool {
        self.config.user.admins.contains(&id)
    }
}
