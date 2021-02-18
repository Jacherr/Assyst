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
    for i in context.assyst.registry.commands.values().filter(|a| a.availability != CommandAvailability::Private) {
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
                .content("<https://discord.com/oauth2/authorize?client_id=571661221854707713&scope=bot>")
                .clone(),
        )
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}
