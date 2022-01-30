use crate::{
    command::{
        command::{
            Argument, Command, CommandAvailability, CommandBuilder, CommandError, ParsedArgument,
            ParsedFlags,
        },
        context::Context,
        registry::CommandResult,
    },
    logger,
    rest::{
        annmarie::{self, info},
        bt::{get_languages, validate_language},
        fake_eval, wsi,
    },
    util::{
        bytes_to_readable, codeblock, ensure_same_guild, exec_sync, extract_page_title,
        format_discord_timestamp, format_time, generate_list, generate_table, get_buffer_filetype,
        get_memory_usage, parse_codeblock,
    },
};
use crate::{
    rest::{bt::translate_single, get_char_info, rust},
    util::{get_current_millis, parse_to_millis},
};
use assyst_common::consts;
use assyst_database::Reminder;
use futures::TryFutureExt;
use lazy_static::lazy_static;
use shared::query_params::ResizeMethod;
use std::{
    collections::HashMap,
    convert::TryInto,
    sync::{atomic::Ordering, Arc},
    time::{Duration, Instant},
};

const USEFUL_LINKS_TEXT: &str = "Invite the bot: <https://jacher.io/assyst>\nSupport server: <https://discord.gg/VRPGgMEhGk>\nVote for Assyst for some sweet perks! <https://vote.jacher.io/topgg> & <https://vote.jacher.io/dbl>";

const CATEGORY_NAME: &str = "misc";

lazy_static! {
    pub static ref PING_COMMAND: Command = CommandBuilder::new("ping")
        .alias("pong")
        .public()
        .description("ping the discord api")
        .cooldown(Duration::from_secs(2))
        .category(CATEGORY_NAME)
        .build();
    pub static ref ENLARGE_COMMAND: Command = CommandBuilder::new("enlarge")
        .flag("url", None)
        .alias("e")
        .public()
        .arg(Argument::ImageBuffer)
        .description("get url of an avatar or emoji")
        .usage("[image]")
        .example(consts::Y21) // you
        .cooldown(Duration::from_secs(2))
        .category(CATEGORY_NAME)
        .build();
    pub static ref HEALTHCHECK_COMMAND: Command = CommandBuilder::new("healthcheck")
        .availability(CommandAvailability::GuildOwner)
        .description("check health of apis assyst uses")
        .cooldown(Duration::from_secs(5))
        .category(CATEGORY_NAME)
        .build();
    pub static ref HELP_COMMAND: Command = CommandBuilder::new("help")
        .arg(Argument::Optional(Box::new(Argument::String)))
        .public()
        .description("get help")
        .usage("<command>")
        .example("caption")
        .cooldown(Duration::from_secs(2))
        .category(CATEGORY_NAME)
        .build();
    pub static ref WSI_STATS_COMMAND: Command = CommandBuilder::new("wsistats")
        .alias("wstat")
        .public()
        .description("get wsi statistics")
        .cooldown(Duration::from_secs(2))
        .category(CATEGORY_NAME)
        .build();
    pub static ref INVITE_COMMAND: Command = CommandBuilder::new("invite")
        .public()
        .description("get bot invite")
        .cooldown(Duration::from_secs(2))
        .category(CATEGORY_NAME)
        .build();
    pub static ref PREFIX_COMMAND: Command = CommandBuilder::new("prefix")
        .arg(Argument::String)
        .availability(CommandAvailability::GuildOwner)
        .description("set the bot prefix in the current guild")
        .usage("[new prefix]")
        .example("¬")
        .cooldown(Duration::from_secs(2))
        .category(CATEGORY_NAME)
        .build();
    pub static ref STATS_COMMAND: Command = CommandBuilder::new("stats")
        .public()
        .description("get bot stats")
        .cooldown(Duration::from_secs(2))
        .category(CATEGORY_NAME)
        .build();
    pub static ref RUST_COMMAND: Command = CommandBuilder::new("rust")
        .arg(Argument::Choice(&["run", "bench", "miri"]))
        .arg(Argument::Choice(&["stable", "beta", "nightly"]))
        .arg(Argument::StringRemaining)
        .public()
        .description("run/benchmark rust code")
        .example("run stable break rust;")
        .usage("[run|bench] [stable|nightly|beta] [code]")
        .cooldown(Duration::from_secs(2))
        .category(CATEGORY_NAME)
        .build();
    pub static ref REMINDER_COMMAND: Command = CommandBuilder::new("remind")
        .arg(Argument::String)
        .arg(Argument::OptionalWithDefaultDynamic(Box::new(Argument::StringRemaining), |_| {
            ParsedArgument::Text(String::from("..."))
        }))
        .alias("reminder")
        .public()
        .description("get reminders or set a reminder, time format is xdyhzm (check examples)")
        .example("1d10h hello")
        .example("44m yea")
        .example("list")
        .example("delete 10")
        .usage("[when|list|delete|remove] <[description|index]>")
        .cooldown(Duration::from_secs(2))
        .category(CATEGORY_NAME)
        .build();
    pub static ref TOP_COMMANDS_COMMAND: Command = CommandBuilder::new("topcmds")
        .arg(Argument::Optional(Box::new(Argument::String)))
        .alias("tcs")
        .public()
        .description("get top command information")
        .cooldown(Duration::from_secs(2))
        .category(CATEGORY_NAME)
        .build();
    pub static ref TOP_BT_COMMAND: Command = CommandBuilder::new("topbtchannel")
        .alias("topbt")
        .availability(CommandAvailability::Private)
        .description("get top btchannel information")
        .cooldown(Duration::from_secs(2))
        .category(CATEGORY_NAME)
        .build();
    pub static ref BT_CHANNEL_COMMAND: Command = CommandBuilder::new("btchannel")
        .arg(Argument::Choice(&["add", "setlanguage", "remove", "languages"]))
        .arg(Argument::Optional(Box::new(Argument::String)))
        .arg(Argument::Optional(Box::new(Argument::String)))
        .availability(CommandAvailability::GuildOwner)
        .description("configures the bad translator feature in this channel")
        .cooldown(Duration::from_secs(10))
        .category(CATEGORY_NAME)
        .usage("[add|setlanguage|remove|languages] <[channel id]> <[language]>")
        .example("add 123456789 en")
        .example("add 123456789 ru")
        .example("languages")
        .example("setlanguage 123456789 fi")
        .example("remove 123456789")
        .build();
    pub static ref COMMAND_COMMAND: Command = CommandBuilder::new("command")
        .alias("cmd")
        .arg(Argument::String)
        .availability(CommandAvailability::GuildOwner)
        .description("toggle enable/disable a command")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref CHARS_COMMAND: Command = CommandBuilder::new("chars")
        .arg(Argument::StringRemaining)
        .alias("char")
        .public()
        .description("get character information of input")
        .example("¬ ¦ y21")
        .usage("[text]")
        .cooldown(Duration::from_secs(2))
        .category(CATEGORY_NAME)
        .build();
    pub static ref TRANSLATE_COMMAND: Command = CommandBuilder::new("translate")
        .arg(Argument::String)
        .arg(Argument::StringRemaining)
        .alias("tr")
        .public()
        .description("translate input text")
        .example("it hello")
        .usage("[language] [text]")
        .cooldown(Duration::from_secs(2))
        .category(CATEGORY_NAME)
        .build();
    pub static ref EXEC_COMMAND: Command = CommandBuilder::new("exec")
        .alias("ex")
        .arg(Argument::StringRemaining)
        .availability(CommandAvailability::Private)
        .description("execute shell command")
        .example("echo hello")
        .usage("[command]")
        .cooldown(Duration::from_secs(2))
        .category(CATEGORY_NAME)
        .build();
    pub static ref FAKE_EVAL_COMMAND: Command = CommandBuilder::new("eval")
        .arg(Argument::StringRemaining)
        .public()
        .description("Evaluate javascript code")
        .example("41 + 1 /* what is it */")
        .usage("[code]")
        .cooldown(Duration::from_secs(1))
        .category(CATEGORY_NAME)
        .build();
    pub static ref PATRON_STATUS_COMMAND: Command = CommandBuilder::new("patronstatus")
        .public()
        .description("Get your patron status")
        .cooldown(Duration::from_secs(1))
        .category(CATEGORY_NAME)
        .build();
    pub static ref CACHE_STATUS_COMMAND: Command = CommandBuilder::new("cachestatus")
        .availability(CommandAvailability::Private)
        .description("Get your patron status")
        .cooldown(Duration::from_secs(1))
        .category(CATEGORY_NAME)
        .build();
}

pub async fn run_ping_command(
    context: Arc<Context>,
    _: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let processing_time = context.metrics.processing_time_start.elapsed().as_micros();
    let start = Instant::now();
    let message = context.reply_with_text("pong!").await?;

    context
        .assyst
        .http
        .update_message(message.channel_id, message.id)
        .content(format!(
            "pong!\nprocessing time: {} µs\nresponse time: {} ms",
            processing_time,
            start.elapsed().as_millis()
        ))?
        .into_future()
        .await?;
    Ok(())
}

pub async fn run_enlarge_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    flags: ParsedFlags,
) -> CommandResult {
    let image = args[0].as_bytes();
    let url = flags.contains_key("url");

    if !url {
        context.reply_with_text("processing...").await?;

        let result = wsi::resize_scale(
            context.assyst.clone(),
            image,
            context.author_id(),
            1.5,
            ResizeMethod::Nearest,
        )
        .await
        .map_err(wsi::format_err)?;

        let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
        context.reply_with_image(format, result).await?;
    } else {
        let format = get_buffer_filetype(&image).unwrap_or_else(|| "png");
        context.reply_with_image(format, image).await?;
    }

    Ok(())
}

pub async fn run_help_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    if args[0].is_nothing() {
        let mut unique_command_names: Vec<&str> = Vec::new();
        let mut command_help_entries: Vec<String> = Vec::new();
        let mut command_categories: HashMap<&str, Vec<&str>> = HashMap::new();

        for i in context
            .assyst
            .registry
            .commands
            .values()
            .filter(|a| a.availability != CommandAvailability::Private)
        {
            if unique_command_names.contains(&&i.name) {
                continue;
            };
            unique_command_names.push(&i.name);
            let category = command_categories.get_mut(i.category);
            match category {
                Some(c) => c.push(&i.name),
                None => {
                    command_categories.insert(i.category, vec![&i.name]);
                }
            };
        }

        command_categories.iter().for_each(|(name, commands)| {
            command_help_entries.push(format!("**{}**\n```\n{}\n```", name, commands.join(", ")))
        });

        context
            .reply_with_text(format!(
                "{}\n*Do {}help [command] for more info on a command.*\n{}",
                &command_help_entries.join("\n"),
                context.prefix,
                USEFUL_LINKS_TEXT
            ))
            .await?;
    } else {
        let command_name = args[0].as_text();
        let command = context
            .assyst
            .registry
            .get_command_from_name_or_alias(command_name)
            .ok_or_else(|| "Command not found".to_owned())?;
        let raw_aliases = &*command.aliases.join(", ");
        let aliases = if command.aliases.len() == 0 {
            "None"
        } else {
            raw_aliases
        };
        let table = generate_table(&vec![
            ("Name", &*command.name),
            ("Aliases", aliases),
            ("Description", &*command.metadata.description),
            (
                "Usage",
                &format!(
                    "{}{} {}",
                    context.prefix, &*command.name, &*command.metadata.usage
                ),
            ),
            ("Access", &*command.availability.to_string()),
            (
                "Cooldown",
                &format!("{} seconds", &*command.cooldown_seconds.to_string()),
            ),
        ]);

        let help = if command.metadata.examples.is_empty() {
            codeblock(
                &format!(
                    "{}\nExamples:\n{}",
                    table,
                    format!("{}{}", context.prefix, &*command.name)
                ),
                "yaml",
            )
        } else {
            codeblock(
                &format!(
                    "{}\nExamples:\n{}",
                    table,
                    &*command
                        .metadata
                        .examples
                        .iter()
                        .map(|e| format!("{}{} {}", context.prefix, &*command.name, e))
                        .collect::<Vec<_>>()
                        .join("\n")
                ),
                "yaml",
            )
        };

        context.reply_with_text(help).await?;
    }
    Ok(())
}

pub async fn run_invite_command(
    context: Arc<Context>,
    _: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    context.reply_with_text(USEFUL_LINKS_TEXT).await?;
    Ok(())
}

pub async fn run_prefix_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let new_prefix = args[0].as_text();
    if new_prefix.len() > 14 {
        return Err("Prefixes cannot be longer than 14 characters".into());
    };

    context
        .assyst
        .database
        .set_prefix_for(context.message.guild_id.unwrap().0, new_prefix)
        .await
        .map_err(|e| e.to_string())?;
    context
        .reply_with_text(format!("Prefix set to {}", new_prefix))
        .await?;
    Ok(())
}

pub async fn run_stats_command(
    context: Arc<Context>,
    _: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let guild_count = context.assyst.metrics.get_guild_count();

    let guild_id = context.message.guild_id.unwrap().0;

    let memory = bytes_to_readable(get_memory_usage().unwrap_or(0));
    let commands = context.assyst.registry.get_command_count().to_string();
    let proc_time = (context.assyst.get_average_processing_time().await / 1e3).to_string();
    let events = context.assyst.metrics.get_events();

    let total_command_calls = context
        .assyst
        .database
        .get_command_usage_stats()
        .await
        .map(|e| e.iter().fold(0, |x, y| x + y.uses))
        .unwrap_or(0)
        .to_string();

    let uptime_minutes = context.assyst.uptime().0 as f32 / 1000f32 / 60f32;
    let commands_per_minute =
        context.assyst.commands_executed.load(Ordering::Relaxed) as f32 / uptime_minutes;
    // *context.assyst.commands_executed.lock().await as f32 / uptime_minutes;

    let stats_table = generate_table(&[
        ("Guilds", &guild_count.to_string()),
        ("Resident Memory Usage", &memory),
        ("Commands", &commands),
        ("Avg Processing Time", &format!("{:.4}s", proc_time)),
        ("Commands Ran", &total_command_calls),
        (
            "Commands Per Minute",
            &format!("{:.2}", commands_per_minute),
        ),
        ("Events Since Restart", &events.to_string()),
        ("BadTranslator Messages", &{
            let (total, guild) = context
                .assyst
                .database
                .get_badtranslator_message_stats(guild_id)
                .await
                .map_err(|e| e.to_string())?;

            format!("Total: {}, Server: {}", total, guild)
        }),
    ]);

    let assyst_uptime = context.assyst.uptime().format();

    let annmarie_info = info(context.assyst.clone())
        .await
        .map_err(annmarie::format_err)?;
    let annmarie_uptime = format_time(annmarie_info.uptime.floor() as u64 * 1000);

    let wsi_info = wsi::stats(context.assyst.clone())
        .await
        .map_err(wsi::format_err)?;
    let wsi_uptime = format_time(wsi_info.uptime_ms as u64);

    let uptimes_table = generate_table(&[
        ("Assyst", &assyst_uptime),
        ("Annmarie", &annmarie_uptime),
        ("WSI", &wsi_uptime),
    ]);

    context
        .reply_with_text(format!(
            "**Stats:** {} **Uptimes:** {}",
            codeblock(&stats_table, "hs"),
            codeblock(&uptimes_table, "hs")
        ))
        .await?;

    Ok(())
}

pub async fn run_wsi_stats_command(
    context: Arc<Context>,
    _: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let response = wsi::stats(context.assyst.clone())
        .await
        .map_err(wsi::format_err)?;

    let output = format!(
        "**Current Requests:** {}\n**Total Workers:** {}",
        response.current_requests, response.total_workers
    );

    context.reply_with_text(output).await?;
    Ok(())
}

pub async fn run_rust_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let ty = args[0].as_choice();
    let channel = args[1].as_choice();
    let code = parse_codeblock(args[2].as_text(), "rs");

    let result = match ty {
        "run" => rust::run_binary(&context.assyst.reqwest_client, code, channel).await,
        "bench" => rust::run_benchmark(&context.assyst.reqwest_client, code).await,
        "miri" => rust::run_miri(&context.assyst.reqwest_client, code, channel).await,
        _ => unreachable!(),
    };

    let result = result.map_err(|e| e.to_string())?;

    let formatted = result.format();

    context.reply_with_text(codeblock(formatted, "rs")).await?;
    Ok(())
}

pub async fn run_remind_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let time = args[0].as_text();

    match time {
        "list" => {
            let user_id = context.message.author.id.0;

            // If the first argument is "list", then we want to fetch a list of reminders
            let reminders = context
                .assyst
                .database
                .fetch_user_reminders(user_id, 10)
                .await
                .map_err(|e| e.to_string())?
                .iter()
                .map(|reminder| {
                    format!(
                        "[#{}] {}: `{}`\n",
                        reminder.id,
                        format_discord_timestamp(reminder.timestamp as u64),
                        reminder.message
                    )
                })
                .collect::<String>();

            let output = if reminders.len() > 0 {
                format!(":calendar: **Upcoming Reminders:**\nThe number on the left side is the reminder ID, use it to delete a reminder: `-remind delete 10`\n\n{}", reminders)
            } else {
                ":calendar: You have no set reminders.".to_owned()
            };

            context.reply_with_text(output).await.unwrap();
            return Ok(());
        }
        "remove" | "delete" => {
            let user_id = context.message.author.id.0;
            let reminder_id = args[1]
                .as_text()
                .parse::<i32>()
                .map_err(|e| e.to_string())?;

            let was_deleted = context
                .assyst
                .database
                .delete_reminder_by_id(user_id, reminder_id)
                .await
                .map_err(|e| e.to_string())?;

            if was_deleted {
                context
                    .reply_with_text(":white_check_mark: Reminder deleted.")
                    .await?;

                return Ok(());
            }

            return Err(CommandError::new_boxed("Failed to delete reminder, either because the ID is wrong or the reminder is not yours."));
        }
        _ => {}
    };

    let comment = match args.get(1) {
        Some(ParsedArgument::Text(arg)) => arg,
        _ => return Err(CommandError::new_boxed("No comment provided")),
    };

    let time: i64 = parse_to_millis(time)
        .map_err(|e| e.as_str())?
        .try_into()
        .map_err(|_| "Input is too large to fit in i64")?;

    if time <= 0 {
        return Err(CommandError::new_boxed("An invalid time was provided"));
    }

    let guild_id = match context.message.guild_id {
        Some(id) => id.0,
        None => {
            return Err(CommandError::new_boxed(
                "This command can only be run in a server",
            ))
        }
    };

    let ftime = format_time(time as u64);
    let time = (get_current_millis() as i64) + time;

    context
        .assyst
        .database
        .add_reminder(Reminder {
            channel_id: context.message.channel_id.0 as i64,
            message: comment.to_owned(),
            message_id: context.message.id.0 as i64,
            user_id: context.message.author.id.0 as i64,
            timestamp: time as i64,
            guild_id: guild_id as i64,
        })
        .await
        .map_err(|e| e.to_string())?;

    context
        .reply_with_text(format!("Reminder set for {} from now", ftime))
        .await?;

    Ok(())
}

pub async fn run_top_commands_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    if args[0].is_nothing() {
        let top_commands = context
            .assyst
            .database
            .get_command_usage_stats()
            .await
            .map_err(|e| e.to_string())?;

        let top_commands_formatted_raw: Vec<(&str, String)> = top_commands
            .iter()
            .take(20)
            .map(|t| (&t.command_name[..], t.uses.to_string()))
            .collect::<Vec<_>>();

        let top_commands_formatted = top_commands_formatted_raw
            .iter()
            .map(|(a, b)| (*a, &b[..]))
            .collect::<Vec<_>>();

        let table = generate_list("Command", "Uses", &top_commands_formatted);

        context
            .reply_with_text(codeblock(&table, "hs"))
            .await
            .map_err(|e| e.to_string())?;
    } else {
        let command_name = args[0].as_text();

        let command = context
            .assyst
            .registry
            .get_command_from_name_or_alias(command_name);

        if let Some(cmd) = command {
            let cmd_name = cmd.name;
            let data = context
                .assyst
                .database
                .get_command_usage_stats_for(cmd_name)
                .await
                .map_err(|e| e.to_string())?;

            context
                .reply_with_text(format!(
                    "Command `{}` been used **{}** times.",
                    cmd_name, data.uses
                ))
                .await
                .map_err(|e| e.to_string())?;
        } else {
            return Err(CommandError::new_boxed(
                "No command with this name or alias exists.",
            ));
        }
    }
    Ok(())
}

pub async fn run_top_bt_command(
    context: Arc<Context>,
    _: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let top_bt = context
        .assyst
        .database
        .get_badtranslator_messages_raw()
        .await
        .map_err(|e| e.to_string())?;

    let top_bt_formatted_raw: Vec<(String, String)> = top_bt
        .iter()
        .take(30)
        .map(|t| (t.guild_id.to_string(), t.message_count.to_string()))
        .collect::<Vec<_>>();

    let top_commands_formatted = top_bt_formatted_raw
        .iter()
        .map(|(a, b)| (&a[..], &b[..]))
        .collect::<Vec<_>>();

    let table = generate_list("Guild ID", "Messages", &top_commands_formatted);

    context
        .reply_with_text(codeblock(&table, "hs"))
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn run_btchannel_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let mut args = args.iter();
    let ty = args.next().map(|a| a.as_choice()).unwrap();

    // safe to unwrap - we can only use it in a guild
    let guild_id = context.message.guild_id.unwrap().0;

    match ty {
        "add" => {
            let channel_id = args
                .next()
                .and_then(|a| a.maybe_text())
                .map(str::parse::<u64>)
                .unwrap_or(Ok(context.message.channel_id.0))
                .map_err(|e| e.to_string())?;

            ensure_same_guild(&context, channel_id, guild_id).await?;

            let language = args.next().and_then(|a| a.maybe_text()).unwrap_or("en");

            let is_valid_language = validate_language(&context.assyst.reqwest_client, language)
                .await
                .map_err(|e| e.to_string())?;

            if !is_valid_language {
                return Err(CommandError::new_boxed(&format!(
                    "This language does not exist or cannot be used as a target language. Run `{}btchannel languages` for a list of languages",
                    context.prefix
                )));
            }

            context
                .http()
                .create_webhook(channel_id.into(), "Bad Translator")
                .await
                .map_err(|e| e.to_string())?;

            context.assyst.database.add_bt_channel(channel_id, language)
                .await
                .map_err(|e| {
                    eprintln!("{:?}", e);
                    CommandError::new_boxed("Registering BT channel failed. This is likely a bug. Please contact one of the bot developers")
                })?;

            context
                .assyst
                .badtranslator
                .add_channel(channel_id, language)
                .await;

            context.reply_with_text("BT Channel registered.").await?
        }
        "setlanguage" => {
            let channel_id = args
                .next()
                .and_then(|a| a.maybe_text())
                .map(str::parse::<u64>)
                .unwrap_or(Ok(context.message.channel_id.0))
                .map_err(|e| e.to_string())?;

            ensure_same_guild(&context, channel_id, guild_id).await?;

            let language = args.next().and_then(|a| a.maybe_text()).unwrap_or("en");

            let is_valid_language = validate_language(&context.assyst.reqwest_client, language)
                .await
                .map_err(|e| e.to_string())?;

            if !is_valid_language {
                return Err(CommandError::new_boxed(&format!(
                    "This language does not exist or cannot be used as a target language. Run `{}btchannel languages` for a list of languages",
                    context.prefix
                )));
            }

            let did_update = context
                .assyst
                .database
                .update_bt_channel_language(channel_id, language)
                .await
                .map_err(|e| e.to_string())?;

            if !did_update {
                return Err(CommandError::new_boxed("Failed to update BT language. Make sure that the provided ID is a valid BT channel."));
            }

            context
                .assyst
                .badtranslator
                .set_channel_language(channel_id, language)
                .await;

            context
                .reply_with_text(format!("BT Channel language set to `{}`", language))
                .await?
        }
        "remove" => {
            let channel_id = args
                .next()
                .and_then(|a| a.maybe_text())
                .map(str::parse::<u64>)
                .ok_or_else(|| String::from("Please provide a channel ID."))?
                .map_err(|e| e.to_string())?;

            ensure_same_guild(&context, channel_id, guild_id).await?;

            let did_delete = context
                .assyst
                .database
                .delete_bt_channel(channel_id)
                .await
                .map_err(|e| e.to_string())?;

            if !did_delete {
                return Err(CommandError::new_boxed(
                    "Failed to delete BT channel. Is it registered in that channel?",
                ));
            }

            context
                .assyst
                .badtranslator
                .remove_bt_channel(channel_id)
                .await;

            context
                .reply_with_text("BT channel successfully deleted.")
                .await?
        }
        "languages" => {
            let languages = get_languages(&context.assyst.reqwest_client)
                .await
                .map_err(|e| e.to_string())?;

            let message = codeblock(&generate_list("Code", "Language", &languages), "hs");
            context.reply_with_text(message).await?
        }
        _ => unreachable!(),
    };

    Ok(())
}

pub async fn run_chars_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let arg = args[0].as_text();

    let chars = arg.chars().take(10);

    let mut output = String::new();

    for ch in chars {
        let (html, url) = get_char_info(&context.assyst.reqwest_client, ch)
            .await
            .map_err(|e| e.to_string())?;

        let title = extract_page_title(&html).unwrap_or_else(|| "<unknown>".to_owned());

        output.push_str(&format!("`{}` — **{}** ({})\n", ch, title, url));
    }

    context.reply_with_text(output).await?;

    Ok(())
}

pub async fn run_translate_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let lang = args[0].as_text();
    let text = args[1].as_text();

    let translation = translate_single(&context.assyst.reqwest_client, text, lang).await?;

    context.reply_with_text(translation.result.text).await?;

    Ok(())
}

pub async fn run_fake_eval_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let code = args[0].as_text();

    let mut response = fake_eval(&context.assyst.reqwest_client, code).await?;

    if response.message.trim() == "42" {
        response.message = "The answer to life, the universe, and everything".to_owned();
    }

    let codeblocked_input = codeblock(code, "js");
    let codeblocked_output = codeblock(&response.message, "js");

    logger::info(
        &context.assyst,
        &format!(
            "User Evaled: {} Output: {}",
            codeblocked_input, codeblocked_output
        ),
    )
    .await;

    context.reply_with_text(codeblocked_output).await?;

    Ok(())
}
pub async fn run_exec_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let command = args[0].as_text();

    let result = exec_sync(command).map_err(|e| e.to_string())?;

    let mut output = "".to_owned();
    if !result.stdout.is_empty() {
        output = format!("`stdout`: ```{}```\n", result.stdout);
    }
    if !result.stderr.is_empty() {
        output = format!("{}`stderr`: ```{}```", output, result.stderr);
    }

    context.reply_with_text(output).await?;

    Ok(())
}

pub async fn run_command_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let command = args[0].as_text();

    let found_command = context
        .assyst
        .registry
        .get_command_from_name_or_alias(command);
    match found_command {
        Some(cmd) => {
            // Because guild disabled commands get cached while doing the initial
            // handle, it is safe to assume that the disabled commands for this guild
            // are cached, and more importantly up-to-date.
            // We can use the cache instead of a DB call to determine whether
            // to enable or disable this command.
            let guild_id = context.message.guild_id.unwrap();
            let lock = context.assyst.database.cache.read().await;
            let disabled_commands = (*lock).disabled_commands.get(&guild_id).unwrap();
            let should_disable = if disabled_commands.contains(cmd.name) {
                false
            } else {
                true
            };
            drop(lock);

            if should_disable {
                context
                    .assyst
                    .database
                    .add_disabled_command(guild_id, cmd.name)
                    .await
                    .map_err(|e| e.to_string())?;
                context
                    .reply_with_text(format!("Disabled command: `{}`", cmd.name))
                    .await
                    .map_err(|e| e.to_string())?;
            } else {
                context
                    .assyst
                    .database
                    .remove_disabled_command(guild_id, cmd.name)
                    .await
                    .map_err(|e| e.to_string())?;
                context
                    .reply_with_text(format!("Enabled command: `{}`", cmd.name))
                    .await
                    .map_err(|e| e.to_string())?;
            };
        }
        None => {
            return Err(CommandError::new_boxed(
                "No command with this name or alias exists.",
            ));
        }
    };

    Ok(())
}

pub async fn run_patron_status_command(
    context: Arc<Context>,
    _: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let lock = context.assyst.patrons.read().await;
    let user = lock.iter().find(|i| i.user_id == context.message.author.id);
    let patron_text;
    let free_requests_text;

    match user {
        Some(u) => {
            patron_text = format!("You're a tier {} patron.", u.tier);
        }
        None => {
            patron_text = String::from(
                "You're not a patron. You can become one at <https://patreon.com/jacher>.",
            );
        }
    }

    let user_free_requests = context
        .assyst
        .database
        .get_user_free_tier1_requests(context.author_id().0 as i64)
        .await?;

    if user_free_requests == 0 {
        free_requests_text = String::from("You don't have any free elevated voting image command invocations. You can vote at <https://vote.jacher.io/topgg> and <https://vote.jacher.io/dbl>.")
    } else {
        free_requests_text = format!(
            "You have {} free elevated voting image command invocations.",
            user_free_requests
        );
    }

    context
        .reply_with_text(format!("{}\n{}", patron_text, free_requests_text))
        .await?;

    Ok(())
}

pub async fn run_cache_status_command(
    context: Arc<Context>,
    _: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let replies_size = context.assyst.replies.read().await.size();
    let ratelimits_size = context.assyst.command_ratelimits.read().await.size();
    context
        .reply_with_text(format!(
            "Replies: {}\nRatelimits: {}",
            replies_size, ratelimits_size
        ))
        .await?;
    Ok(())
}

pub async fn run_healthcheck_command(
    context: Arc<Context>,
    _: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let healthcheck = context.assyst.healthcheck_result.lock().await;
    let elapsed = healthcheck.0.elapsed().as_secs();
    let healthcheck = healthcheck.1.clone();
    if healthcheck.is_empty() {
        context
            .reply_with_text(format!(
                "No healthcheck results found.\nElapsed: {} seconds",
                elapsed
            ))
            .await?;

        return Ok(());
    }

    let fmt = healthcheck
        .iter()
        .map(|x| (x.service.clone(), x.status.to_string()))
        .collect::<Vec<_>>();

    let output = generate_table(&fmt.iter().map(|x| (&x.0[..], &x.1[..])).collect::<Vec<_>>());

    context
        .reply_with_text(format!(
            "Updated {} seconds ago\n{}",
            elapsed,
            codeblock(&output, "ansi")
        ))
        .await?;

    Ok(())
}
