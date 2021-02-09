use crate::{box_str, command::{command::{Argument, Command, CommandAvailability, CommandMetadata, ParsedArgument, force_as}, context::Context, messagebuilder::MessageBuilder, registry::CommandResult}, consts::WORKING_FILESIZE_LIMIT_BYTES, rest::wsi};
use bytes::Bytes;
use lazy_static::lazy_static;
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

async fn compress_if_large(context: Arc<Context>, image: Bytes) -> Result<Bytes, String> {
    let five_mb = 5000000;
    if image.len() > WORKING_FILESIZE_LIMIT_BYTES {
        let comparator = image.len() - WORKING_FILESIZE_LIMIT_BYTES;
        let fuzz_level = comparator / five_mb;
        context.reply_with_text("compressing...").await?;
        wsi::compress(context.assyst.clone(), image, fuzz_level)
            .await
            .map_err(wsi::format_err)
    } else {
        Ok(image)
    }
}

pub async fn run_caption_command(context: Arc<Context>, mut args: Vec<ParsedArgument>) -> CommandResult {
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let image = compress_if_large(context.clone(), raw_image).await?;
    let text = force_as::text(&args[0]);
    context.reply_with_text("processing...").await?;
    let result = wsi::caption(context.assyst.clone(), image, text).await
        .map_err(wsi::format_err)?;
    let format = get_buffer_filetype(&result)
        .unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

pub async fn run_reverse_command(context: Arc<Context>, mut args: Vec<ParsedArgument>) -> CommandResult {
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let image = compress_if_large(context.clone(), raw_image).await?;
    context.reply_with_text("processing...").await?;
    let result = wsi::reverse(context.assyst.clone(), image).await
        .map_err(wsi::format_err)?;
    let format = get_buffer_filetype(&result)
        .unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

pub async fn run_spin_command(context: Arc<Context>, mut args: Vec<ParsedArgument>) -> CommandResult {
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let image = compress_if_large(context.clone(), raw_image).await?;
    context.reply_with_text("processing...").await?;
    let result = wsi::spin(context.assyst.clone(), image).await
        .map_err(wsi::format_err)?;
    let format = get_buffer_filetype(&result)
        .unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}