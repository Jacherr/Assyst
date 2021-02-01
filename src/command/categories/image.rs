use crate::{box_str, command::{command::{Argument, Command, CommandAvailability, CommandMetadata, ParsedArgument, force_as}, context::Context, messagebuilder::MessageBuilder, registry::CommandResult}};
use lazy_static::lazy_static;
use std::sync::Arc;

lazy_static!{
    pub static ref CAPTION_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![Argument::ImageBuffer, Argument::StringRemaining],
        availability: CommandAvailability::Private,
        metadata: CommandMetadata {
            description: box_str!("add a caption to an image"),
            examples: vec![],
            usage: box_str!("[image] [caption]")
        },
        name: box_str!("caption")
    };
}

pub async fn run_caption_command(context: Arc<Context>, args: Vec<ParsedArgument>) -> CommandResult {
    context.reply(MessageBuilder::new().content(&format!("{:?}", args)).clone())
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}