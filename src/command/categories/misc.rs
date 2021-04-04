use crate::{
    command::{
        command::{
            force_as, Argument, Command, CommandAvailability, CommandMetadata, ParsedArgument,
        },
        context::Context,
        messagebuilder::MessageBuilder,
        registry::CommandResult,
    },
    util::{
        codeblock, extract_page_title, format_time, generate_list, generate_table,
        get_memory_usage, parse_codeblock,
    },
};
use crate::{
    database::Reminder,
    rest::{get_char_info, rust},
    util::{get_current_millis, parse_to_millis},
};
use futures::TryFutureExt;
use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Arc, time::Instant};

lazy_static! {
    pub static ref PING_COMMAND: Command = Command {
        aliases: vec!["pong"],
        args: vec![],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: "Get Discord WebSocket and REST ping",
            examples: vec![],
            usage: ""
        },
        name: "ping",
        cooldown_seconds: 2,
        category: "misc"
    };
    pub static ref ENLARGE_COMMAND: Command = Command {
        aliases: vec!["e"],
        args: vec![Argument::ImageUrl],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: "enlarge an image",
            examples: vec!["312715611413413889"],
            usage: "[image]"
        },
        name: "enlarge",
        cooldown_seconds: 2,
        category: "misc"
    };
    pub static ref HELP_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![Argument::Optional(Box::new(Argument::String))],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: "get help",
            examples: vec!["", "caption"],
            usage: "<command>"
        },
        name: "help",
        cooldown_seconds: 2,
        category: "misc"
    };
    pub static ref INVITE_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: "get bot invite",
            examples: vec![],
            usage: ""
        },
        name: "invite",
        cooldown_seconds: 2,
        category: "misc"
    };
    pub static ref PREFIX_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![Argument::String],
        availability: CommandAvailability::GuildOwner,
        metadata: CommandMetadata {
            description: "set bot prefix",
            examples: vec!["-"],
            usage: "[new prefix]"
        },
        name: "prefix",
        cooldown_seconds: 2,
        category: "misc"
    };
    pub static ref STATS_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: "get bot statistics",
            examples: vec![],
            usage: ""
        },
        name: "stats",
        cooldown_seconds: 2,
        category: "misc"
    };
    pub static ref RUST_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![
            Argument::Choice(&["run", "bench"]),
            Argument::Choice(&["stable", "beta", "nightly"]),
            Argument::StringRemaining
        ],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: "run/benchmark rust code",
            examples: vec!["run stable break rust;"],
            usage: "[run|bench] [stable|nightly|beta] [code]"
        },
        name: "rust",
        cooldown_seconds: 2,
        category: "misc"
    };
    pub static ref REMINDER_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![
            Argument::String,
            Argument::OptionalWithDefaultDynamic(Box::new(Argument::StringRemaining), |_| {
                ParsedArgument::Text(String::from("..."))
            })
        ],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: "set a reminder, time format is xdyhzm (check examples)",
            examples: vec!["1d10h hello", "44m yea"],
            usage: "[when|list] <[description]>"
        },
        name: "remind",
        cooldown_seconds: 2,
        category: "misc"
    };
    pub static ref TOP_COMMANDS_COMMAND: Command = Command {
        aliases: vec!["tcs"],
        args: vec![Argument::Optional(Box::new(Argument::String))],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: "get top command usage info",
            examples: vec![],
            usage: ""
        },
        name: "topcmds",
        cooldown_seconds: 2,
        category: "misc"
    };
    pub static ref BT_CHANNEL_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![],
        availability: CommandAvailability::GuildOwner,
        metadata: CommandMetadata {
            description: "configures the bad translator feature in this channel",
            examples: vec![],
            usage: ""
        },
        name: "btchannel",
        cooldown_seconds: 30,
        category: "misc"
    };
    pub static ref CHARS_COMMAND: Command = Command {
        aliases: vec!["char"],
        args: vec![Argument::StringRemaining],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: "returns character information for given input",
            examples: vec![],
            usage: "[text]"
        },
        name: "chars",
        cooldown_seconds: 1,
        category: "misc"
    };
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
                    "{}\n*Do {}help [command] for more info on a command.*\nInvite the bot: <https://jacher.io/assyst>\nSupport server: <https://discord.gg/JBvJbBEDpA>\n**Note: The default bot prefix is `{}`**",
                    &command_help_entries.join("\n"),
                    context.prefix,
                    context.assyst.config.default_prefix
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
                        "Bot invite: <https://jacher.io/assyst>\nSupport server: <https://discord.gg/JBvJbBEDpA>\n**Note: The default bot prefix is `{}`**", 
                        context.assyst.config.default_prefix
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
    let guilds = context
        .assyst
        .http
        .current_user_guilds()
        .limit(100)
        .map_err(|e| e.to_string())?
        .await
        .map_err(|e| e.to_string())?
        .len()
        .to_string();

    let memory = get_memory_usage().unwrap_or("Unknown".to_owned());
    let commands = context.assyst.registry.get_command_count().to_string();
    let proc_time = (context.assyst.get_average_processing_time().await / 1e3).to_string();

    let table = generate_table(&[
        ("Guilds", &guilds),
        ("Memory", &memory),
        ("Commands", &commands),
        ("Avg Processing Time", &format!("{:.4}s", proc_time)),
        ("Uptime", &context.assyst.uptime().format()),
        ("BadTranslator Messages", &{
            let read_lock = context.assyst.metrics.read().await;
            let total = read_lock.bt_messages.sum();
            let guild_count = context
                .message
                .guild_id
                .and_then(|id| read_lock.bt_messages.0.get(&id.0))
                .unwrap_or(&0);

            format!("Total: {}, Server: {}", total, guild_count)
        }),
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

pub async fn run_rust_command(context: Arc<Context>, args: Vec<ParsedArgument>) -> CommandResult {
    let ty = force_as::choice(&args[0]);
    let channel = force_as::choice(&args[1]);
    let code = force_as::text(&args[2]);

    let result = if ty == "run" {
        rust::run_binary(
            &context.assyst.reqwest_client,
            parse_codeblock(code, "rs"),
            channel,
        )
        .await
    } else {
        rust::run_benchmark(&context.assyst.reqwest_client, parse_codeblock(code, "rs")).await
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

    if time == "list" {
        let user_id = context.message.author.id.0;

        // If the first argument is "list", then we want to fetch a list of reminders
        let reminders = context.assyst.database
            .fetch_user_reminders(user_id, 10)
            .await
            .map_err(|e| e.to_string())?
            .iter()
            .map(|reminder| format!("In {}: `{}`\n", format_time(reminder.timestamp as u64 - get_current_millis()), reminder.message))
            .collect::<String>();

        let output = format!(":calendar: Upcoming Reminders\n\n{}", reminders);
    
        context.reply_with_text(&output).await.unwrap();
        return Ok(());
    }

    let comment = match args.get(1) {
        Some(ParsedArgument::Text(arg)) => arg,
        _ => return Err("No comment provided".to_owned())
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
    _: Vec<ParsedArgument>,
) -> CommandResult {
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
