use crate::{box_str, command::{command::{Argument, Command, CommandAvailability, CommandMetadata, ParsedArgument, force_as}, context::Context, messagebuilder::MessageBuilder, registry::CommandResult}, rest::wsi};
use lazy_static::lazy_static;
use wsi::RequestError;
use std::sync::Arc;
use crate::util::get_buffer_filetype;

lazy_static!{
    pub static ref CAPTION_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![Argument::ImageBuffer, Argument::StringRemaining],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: box_str!("add a caption to an image"),
            examples: vec![],
            usage: box_str!("[image] [caption]")
        },
        name: box_str!("caption")
    };

    pub static ref REVERSE_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![Argument::ImageBuffer],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: box_str!("reverse a gif"),
            examples: vec![],
            usage: box_str!("[image]")
        },
        name: box_str!("reverse")
    };

    pub static ref SPIN_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![Argument::ImageBuffer],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: box_str!("spin an image"),
            examples: vec![],
            usage: box_str!("[image]")
        },
        name: box_str!("spin")
    };
}

pub async fn run_caption_command(context: Arc<Context>, mut args: Vec<ParsedArgument>) -> CommandResult {
    let image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let text = force_as::text(&args[0]);
    let result = wsi::caption(context.assyst.clone(), image, text).await
        .map_err(|err| {
            match err {
                RequestError::Reqwest(e) => e.to_string(),
                RequestError::Wsi(e) => format!("Error {}: {}", e.code, e.message)
            }
        })?;
    let format = get_buffer_filetype(&result)
        .unwrap_or_else(|| "png");
    context.reply_with_image(format, result)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn run_reverse_command(context: Arc<Context>, mut args: Vec<ParsedArgument>) -> CommandResult {
    let image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let result = wsi::reverse(context.assyst.clone(), image).await
        .map_err(|err| {
            match err {
                RequestError::Reqwest(e) => e.to_string(),
                RequestError::Wsi(e) => format!("Error {}: {}", e.code, e.message)
            }
        })?;
    let format = get_buffer_filetype(&result)
        .unwrap_or_else(|| "png");
    context.reply_with_image(format, result)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn run_spin_command(context: Arc<Context>, mut args: Vec<ParsedArgument>) -> CommandResult {
    let image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let result = wsi::spin(context.assyst.clone(), image).await
        .map_err(|err| {
            match err {
                RequestError::Reqwest(e) => e.to_string(),
                RequestError::Wsi(e) => format!("Error {}: {}", e.code, e.message)
            }
        })?;
    let format = get_buffer_filetype(&result)
        .unwrap_or_else(|| "png");
    context.reply_with_image(format, result)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}