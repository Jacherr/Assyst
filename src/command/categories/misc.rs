use crate::{box_str, command::{command::{Argument, Command, CommandAvailability, CommandMetadata, ParsedArgument, force_as}, context::Context, messagebuilder::MessageBuilder, registry::CommandResult}};
use lazy_static::lazy_static;
use std::sync::Arc;

lazy_static!{
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
        availability: CommandAvailability::Private,
        metadata: CommandMetadata {
            description: box_str!("enlarge an image"),
            examples: vec![],
            usage: box_str!("[image]")
        },
        name: box_str!("enlarge")
    };
}

pub async fn run_ping_command(context: Arc<Context>, _: Vec<ParsedArgument>) -> CommandResult {
    context.reply(MessageBuilder::new().content("pong!").clone())
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn run_enlarge_command(context: Arc<Context>, args: Vec<ParsedArgument>) -> CommandResult {
    let url = force_as::text(&args[0]);
    context.reply(MessageBuilder::new().content(url).clone())
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}