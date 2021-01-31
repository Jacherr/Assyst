use crate::{box_str, command::{command::{Command, CommandAvailability, CommandMetadata, Argument, ParsedArgument}, context::Context, messagebuilder::MessageBuilder, registry::CommandResult}};
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
    pub static ref TEST_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![Argument::ImageUrl],
        availability: CommandAvailability::Private,
        metadata: CommandMetadata {
            description: box_str!("test"),
            examples: vec![],
            usage: box_str!("test")
        },
        name: box_str!("test")
    };
}

pub async fn run_ping_command(context: Arc<Context>, _: Vec<ParsedArgument>) -> CommandResult {
    context.reply(MessageBuilder::new().content("pong!").clone())
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn run_test_command(context: Arc<Context>, args: Vec<ParsedArgument>) -> CommandResult {
    context.reply(MessageBuilder::new().content(&format!("{:?}", args)).clone())
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}