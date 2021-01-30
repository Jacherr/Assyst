use crate::{box_str, command::{command::{CommandAvailability, CommandMetadata, Argument, Command}, context::Context, messagebuilder::MessageBuilder, registry::{CommandResult, CommandResultOuter}}};
use lazy_static::lazy_static;

lazy_static!{
    pub static ref PingCommand: Command = Command {
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
}

pub async fn run_ping_command(context: Context, args: Vec<String>) -> CommandResult {
    context.reply(MessageBuilder::new().content("pong!").clone()).await;
    Ok(())
}