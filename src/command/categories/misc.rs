use crate::{
    box_str,
    command::{
        command::{
            force_as, Argument, Command, CommandAvailability, CommandMetadata, ParsedArgument,
        },
        context::Context,
        messagebuilder::MessageBuilder,
        registry::CommandResult,
    },
    util::{codeblock, generate_table, get_memory_usage, parse_codeblock},
};
use crate::{
    database::Reminder,
    rest::rust,
    util::{get_current_millis, parse_to_millis},
};
use futures::TryFutureExt;
use lazy_static::lazy_static;
use std::{sync::Arc, time::Instant};

lazy_static! {
    pub static ref PING_COMMAND: Command = Command {
        aliases: vec![box_str!("pong")],
        args: vec![],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: box_str!("Get Discord WebSocket and REST ping"),
            examples: vec![],
            usage: box_str!("")
        },
        name: box_str!("ping"),
    };
    pub static ref ENLARGE_COMMAND: Command = Command {
        aliases: vec![box_str!("e")],
        args: vec![Argument::ImageUrl],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: box_str!("enlarge an image"),
            examples: vec![],
            usage: box_str!("[image]")
        },
        name: box_str!("enlarge")
    };
    pub static ref HELP_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: box_str!("get help"),
            examples: vec![],
            usage: box_str!("")
        },
        name: box_str!("help")
    };
    pub static ref INVITE_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: box_str!("get bot invite"),
            examples: vec![],
            usage: box_str!("")
        },
        name: box_str!("invite")
    };
    pub static ref STATS_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: box_str!("get bot statistics"),
            examples: vec![],
            usage: box_str!("")
        },
        name: box_str!("stats")
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
            description: box_str!("run/benchmark rust code"),
            examples: vec![],
            usage: box_str!("[run|bench] [stable|nightly|beta] [code]")
        },
        name: box_str!("rust")
    };
    pub static ref REMINDER_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![Argument::String, Argument::StringRemaining],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: box_str!("set a reminder"),
            examples: vec![],
            usage: box_str!("[when] [description]")
        },
        name: box_str!("remind")
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

pub async fn run_help_command(context: Arc<Context>, _: Vec<ParsedArgument>) -> CommandResult {
    let mut unique_command_names: Vec<&Box<str>> = Vec::new();
    let mut command_help_entries: Vec<String> = Vec::new();
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
        command_help_entries.push(format!("`{}` - *{}*", i.name, i.metadata.description));
    }
    context
        .reply(
            MessageBuilder::new()
                .content(&format!("{}\nInvite the bot: <https://discord.com/oauth2/authorize?client_id=571661221854707713&scope=bot>", &command_help_entries.join("\n")))
                .clone(),
        )
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn run_invite_command(context: Arc<Context>, _: Vec<ParsedArgument>) -> CommandResult {
    context
        .reply(
            MessageBuilder::new()
                .content(
                    "<https://discord.com/oauth2/authorize?client_id=571661221854707713&scope=bot>",
                )
                .clone(),
        )
        .await
        .map_err(|e| e.to_string())?;
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
    let comment = force_as::text(&args[1]);

    let time = parse_to_millis(time).map_err(|e| e.to_string())? as u64;

    // TODO: check if time is too large

    let guild_id = match context.message.guild_id {
        Some(id) => id.0,
        None => return Err("This command can only be run in a server".to_owned()),
    };

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
        .reply(MessageBuilder::new().content("Reminder set."))
        .await
        .map_err(|e| e.to_string())
        .and_then(|_| Ok(()))
}
