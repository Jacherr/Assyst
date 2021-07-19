use crate::{
    command::{
        command::{
            force_as, Argument, Command, CommandAvailability, CommandBuilder, ParsedArgument,
        },
        context::Context,
        messagebuilder::MessageBuilder,
        registry::CommandResult,
    },
    rest::{
        annmarie::{self, info},
        fake_eval, wsi,
    },
    util::{
        codeblock, exec_sync, extract_page_title, format_time, generate_list, generate_table,
        get_memory_usage, parse_codeblock,
    },
};
use crate::{
    consts::Y21,
    database::Reminder,
    rest::{bt::translate_single, get_char_info, rust},
    util::{get_current_millis, parse_to_millis},
};
use futures::TryFutureExt;
use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};

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
        .alias("e")
        .public()
        .arg(Argument::ImageUrl)
        .description("get url of an avatar or emoji")
        .usage("[image]")
        .example(Y21) // you
        .cooldown(Duration::from_secs(2))
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
        .availability(CommandAvailability::GuildOwner)
        .description("configures the bad translator feature in this channel")
        .cooldown(Duration::from_secs(2))
        .category(CATEGORY_NAME)
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
    pub static ref WSI_RESTART_COMMAND: Command = CommandBuilder::new("wsirestart")
        .availability(CommandAvailability::Private)
        .description("Schedule a wsi restart for when no jobs are active")
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
    pub static ref UPTIME_COMMAND: Command = CommandBuilder::new("uptime")
        .availability(CommandAvailability::Private)
        .description("Get the uptime of Assyst services")
        .cooldown(Duration::from_secs(1))
        .category(CATEGORY_NAME)
        .build();
}

pub async fn run_ping_command(context: Arc<Context>, _: Vec<ParsedArgument>) -> CommandResult {
    let processing_time = context.metrics.processing_time_start.elapsed().as_micros();
    let start = Instant::now();
    let message = context
        .reply(MessageBuilder::new().content("pong!").clone())
        .await
        .map_err(|e| e.to_string())?;
    context
        .assyst
        .http
        .update_message(message.channel_id, message.id)
        .content(format!(
            "pong!\nprocessing time: {}µs\nresponse time:{}ms",
            processing_time,
            start.elapsed().as_millis()
        ))
        .map_err(|e| e.to_string())?
        .into_future()
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn run_enlarge_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
) -> CommandResult {
    let url = force_as::text(&args[0]);
    context
        .reply(MessageBuilder::new().content(url).clone())
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn run_help_command(context: Arc<Context>, args: Vec<ParsedArgument>) -> CommandResult {
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
            .reply_with_text(
                &format!(
                    "{}\n*Do {}help [command] for more info on a command.*\nInvite the bot: <https://jacher.io/assyst>\nSupport server: <https://discord.gg/VRPGgMEhGkg>\n**Note: The default bot prefix is `{}`**",
                    &command_help_entries.join("\n"),
                    context.prefix,
                    context.assyst.config.prefix.default
                )
            )
            .await
            .map_err(|e| e.to_string())?;
    } else {
        let command_name = force_as::text(&args[0]);
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
        let help;
        if command.metadata.examples.len() == 0 {
            help = codeblock(
                &format!(
                    "{}\nExamples:\n{}",
                    table,
                    format!("{}{}", context.prefix, &*command.name)
                ),
                "yaml",
            );
        } else {
            help = codeblock(
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
            );
        }
        context
            .reply_with_text(&help)
            .await
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

pub async fn run_invite_command(context: Arc<Context>, _: Vec<ParsedArgument>) -> CommandResult {
    context
        .reply(
            MessageBuilder::new()
                .content(
                    &format!(
                        "Bot invite: <https://jacher.io/assyst>\nSupport server: <https://discord.gg/VRPGgMEhGkg>\n**Note: The default bot prefix is `{}`**", 
                        context.assyst.config.prefix.default
                    ),
                )
                .clone(),
        )
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn run_prefix_command(context: Arc<Context>, args: Vec<ParsedArgument>) -> CommandResult {
    let new_prefix = force_as::text(&args[0]);
    if new_prefix.len() > 14 {
        context
            .reply_err("Prefixes cannot be longer than 14 characters")
            .map_err(|e| e.to_string())
            .await?;
        return Ok(());
    };
    context
        .assyst
        .database
        .set_prefix_for(context.message.guild_id.unwrap().0, new_prefix)
        .await
        .map_err(|e| e.to_string())?;
    context
        .reply_with_text(&format!("Prefix set to {}", new_prefix))
        .await?;
    Ok(())
}

pub async fn run_stats_command(context: Arc<Context>, _: Vec<ParsedArgument>) -> CommandResult {
    let app = crate::rest::maryjane::get_application(
        context.assyst.clone(),
        context.assyst.config.bot_id,
    )
    .await;

    let guild_count = match app {
        Ok(a) => a.bot.guild_count.to_string(),
        Err(e) => format!("failed to fetch: {}", crate::rest::annmarie::format_err(e)),
    };

    let guild_id = context.message.guild_id.unwrap().0;

    let memory = get_memory_usage().unwrap_or("Unknown".to_owned());
    let commands = context.assyst.registry.get_command_count().to_string();
    let proc_time = (context.assyst.get_average_processing_time().await / 1e3).to_string();

    let total_command_calls = context
        .assyst
        .database
        .get_command_usage_stats()
        .await
        .map(|e| e.iter().fold(0, |x, y| x + y.uses))
        .unwrap_or(0)
        .to_string();

    let table = generate_table(&[
        ("Guilds", &guild_count),
        ("Memory", &memory),
        ("Commands", &commands),
        ("Avg Processing Time", &format!("{:.4}s", proc_time)),
        ("Uptime", &context.assyst.uptime().format()),
        ("BadTranslator Messages", &{
            let (total, guild) = context
                .assyst
                .database
                .get_badtranslator_message_stats(guild_id)
                .await
                .map_err(|e| e.to_string())?;

            format!("Total: {}, Server: {}", total, guild)
        }),
        ("Commands Ran", &total_command_calls),
    ]);

    context
        .reply(
            MessageBuilder::new()
                .content(&codeblock(&table, "hs"))
                .clone(),
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub async fn run_wsi_stats_command(context: Arc<Context>, _: Vec<ParsedArgument>) -> CommandResult {
    let response = wsi::stats(context.assyst.clone())
        .await
        .map_err(wsi::format_err)?;

    let output = format!(
        "**Current Requests:** {}\n**Total Workers:** {}",
        response.current_requests, response.total_workers
    );

    context
        .reply_with_text(&output)
        .await
        .map_err(|e| e.to_string())
        .map(|_| ())
}

pub async fn run_rust_command(context: Arc<Context>, args: Vec<ParsedArgument>) -> CommandResult {
    let ty = force_as::choice(&args[0]);
    let channel = force_as::choice(&args[1]);
    let code = parse_codeblock(force_as::text(&args[2]), "2rs");

    let result = match ty {
        "run" => rust::run_binary(&context.assyst.reqwest_client, code, channel).await,
        "bench" => rust::run_benchmark(&context.assyst.reqwest_client, code).await,
        "miri" => rust::run_miri(&context.assyst.reqwest_client, code, channel).await,
        _ => unreachable!(),
    };

    let result = result.map_err(|e| e.to_string())?;

    let formatted = result.format();

    context
        .reply(MessageBuilder::new().content(&codeblock(formatted, "rs")))
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub async fn run_remind_command(context: Arc<Context>, args: Vec<ParsedArgument>) -> CommandResult {
    let time = force_as::text(&args[0]);

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
                        "[#{}] <t:{}:R>: `{}`\n",
                        reminder.id,
                        reminder.timestamp / 1000,
                        reminder.message
                    )
                })
                .collect::<String>();

            let output = if reminders.len() > 0 {
                format!(":calendar: **Upcoming Reminders:**\nThe number on the left side is the reminder ID, use it to delete a reminder: `-remind delete 10`\n\n{}", reminders)
            } else {
                ":calendar: You have no set reminders.".to_owned()
            };

            context.reply_with_text(&output).await.unwrap();
            return Ok(());
        }
        "remove" | "delete" => {
            let user_id = context.message.author.id.0;
            let reminder_id = force_as::text(&args[1])
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

            return Err("Failed to delete reminder, either because the ID is wrong or the reminder is not yours.".to_owned());
        }
        _ => {}
    };

    let comment = match args.get(1) {
        Some(ParsedArgument::Text(arg)) => arg,
        _ => return Err("No comment provided".to_owned()),
    };

    let time = parse_to_millis(time).map_err(|e| e.to_string())? as u64;

    if time == 0 {
        return Err("An invalid time was provided".to_owned());
    }

    let guild_id = match context.message.guild_id {
        Some(id) => id.0,
        None => return Err("This command can only be run in a server".to_owned()),
    };

    let ftime = format_time(time);
    let time = get_current_millis() + time;

    // TODO: try_into
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
        .reply_with_text(&format!("Reminder set for {} from now", ftime))
        .await
        .map_err(|e| e.to_string())
        .and_then(|_| Ok(()))
}

pub async fn run_top_commands_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
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
            .reply_with_text(&codeblock(&table, "hs"))
            .await
            .map_err(|e| e.to_string())?;
    } else {
        let command_name = force_as::text(&args[0]);

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
                .reply_with_text(&format!(
                    "Command `{}` been used **{}** times.",
                    cmd_name, data.uses
                ))
                .await
                .map_err(|e| e.to_string())?;
        } else {
            return Err("No command with this name or alias exists.".to_owned());
        }
    }
    Ok(())
}

pub async fn run_top_bt_command(context: Arc<Context>, _: Vec<ParsedArgument>) -> CommandResult {
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
        .reply_with_text(&codeblock(&table, "hs"))
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn run_btchannel_command(context: Arc<Context>, _: Vec<ParsedArgument>) -> CommandResult {
    let channel_id = context.message.channel_id;

    context
        .http()
        .create_webhook(channel_id, "Bad Translator")
        .await
        .map_err(|e| e.to_string())?;

    context.assyst.database.add_bt_channel(channel_id.0)
        .await
        .map_err(|e| {
            eprintln!("{:?}", e);
            "Registering BT channel failed. This is likely a bug. Please contact one of the bot developers".to_string()
        })?;

    context.assyst.badtranslator.add_channel(channel_id.0).await;

    context
        .reply_with_text("ok")
        .await
        .map_err(|e| e.to_string())
        .map(|_| ())
}

pub async fn run_chars_command(context: Arc<Context>, args: Vec<ParsedArgument>) -> CommandResult {
    let arg = force_as::text(&args[0]);

    let chars = arg.chars().take(10);

    let mut output = String::new();

    for ch in chars {
        let (html, url) = get_char_info(&context.assyst.reqwest_client, ch)
            .await
            .map_err(|e| e.to_string())?;

        let title = extract_page_title(&html).unwrap_or_else(|| "<unknown>".to_owned());

        output.push_str(&format!("`{}` — **{}** ({})\n", ch, title, url));
    }

    context
        .reply_with_text(&output)
        .await
        .map_err(|e| e.to_string())
        .map(|_| ())
}

pub async fn run_translate_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
) -> CommandResult {
    let lang = force_as::text(&args[0]);
    let text = force_as::text(&args[1]);

    let translation = translate_single(&context.assyst.reqwest_client, text, lang)
        .await
        .map_err(|e| e.to_string())?;

    context
        .reply_with_text(&translation.result.text)
        .await
        .unwrap();

    Ok(())
}

pub async fn run_fake_eval_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
) -> CommandResult {
    let code = force_as::text(&args[0]);

    let mut response = fake_eval(&context.assyst.reqwest_client, code)
        .await
        .map_err(|e| e.to_string())?;

    if response.message.trim() == "42" {
        response.message = "The answer to life, the universe, and everything".to_owned()
    };

    context
        .reply_with_text(&codeblock(&response.message, "js"))
        .await
        .map(|_| ())
}
pub async fn run_exec_command(context: Arc<Context>, args: Vec<ParsedArgument>) -> CommandResult {
    let command = force_as::text(&args[0]);

    let result = exec_sync(command).map_err(|e| e.to_string())?;

    let mut output = "".to_owned();
    if !result.stdout.is_empty() {
        output = format!("`stdout`: ```{}```\n", result.stdout);
    }
    if !result.stderr.is_empty() {
        output = format!("{}`stderr`: ```{}```", output, result.stderr);
    }

    context.reply_with_text(&output).await.unwrap();

    Ok(())
}

pub async fn run_command_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
) -> CommandResult {
    let command = force_as::text(&args[0]);

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
                    .reply_with_text(&format!("Disabled command: `{}`", cmd.name))
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
                    .reply_with_text(&format!("Enabled command: `{}`", cmd.name))
                    .await
                    .map_err(|e| e.to_string())?;
            };
        }
        None => {
            return Err("No command with this name or alias exists.".to_owned());
        }
    };

    Ok(())
}

pub async fn run_wsi_restart_command(
    context: Arc<Context>,
    _: Vec<ParsedArgument>,
) -> CommandResult {
    wsi::restart(context.assyst.clone())
        .await
        .map_err(wsi::format_err)?;
    context
        .reply_with_text("Restart scheduled for when api is idle")
        .await?;
    Ok(())
}

pub async fn run_patron_status_command(
    context: Arc<Context>,
    _: Vec<ParsedArgument>,
) -> CommandResult {
    let lock = context.assyst.patrons.read().await;
    let user = lock.iter().find(|i| i.user_id == context.message.author.id);

    match user {
        Some(u) => {
            context
                .reply_with_text(&format!("You're a tier {} patron.", u.tier))
                .await?;
        }
        None => {
            context
                .reply_with_text(
                    "You're not a patron. You can become one at <https://patreon.com/jacher>",
                )
                .await?;
        }
    }

    Ok(())
}

pub async fn run_cache_status_command(
    context: Arc<Context>,
    _: Vec<ParsedArgument>,
) -> CommandResult {
    let replies_size = context.assyst.replies.read().await.size();
    let ratelimits_size = context.assyst.command_ratelimits.read().await.size();
    context
        .reply_with_text(&format!(
            "Replies: {}\nRatelimits: {}",
            replies_size, ratelimits_size
        ))
        .await?;
    Ok(())
}

pub async fn run_uptime_command(context: Arc<Context>, _: Vec<ParsedArgument>) -> CommandResult {
    let assyst_uptime = context.assyst.uptime().format();

    let annmarie_info = info(context.assyst.clone())
        .await
        .map_err(annmarie::format_err)?;
    let annmarie_uptime = format_time(annmarie_info.uptime.floor() as u64 * 1000);

    let wsi_info = wsi::stats(context.assyst.clone())
        .await
        .map_err(wsi::format_err)?;
    let wsi_uptime = format_time(wsi_info.uptime_ms as u64);

    let output = generate_table(&[
        ("Assyst", &assyst_uptime),
        ("Annmarie", &annmarie_uptime),
        ("WSI", &wsi_uptime),
    ]);

    context.reply_with_text(&codeblock(&output, "yaml")).await?;
    Ok(())
}
