
use crate::util::get_buffer_filetype;
use crate::{
    command::{
        command::{
            Argument, Command, CommandBuilder, 
            ParsedArgument, ParsedFlags,
        },
        context::Context,
        registry::CommandResult,
    },
    rest::wsi,
};
use assyst_common::consts;
use bytes::Bytes;
use lazy_static::lazy_static;
use std::sync::Arc;
use std::time::Duration;

use super::image_wsi::run_wsi_noarg_command;

const CATEGORY_NAME: &str = "image (makesweet)";

lazy_static! {
    pub static ref BILLBOARD_COMMAND: Command = CommandBuilder::new("billboard")
        .arg(Argument::ImageBuffer)
        .public()
        .description("display an image on a billboard")
        .example(consts::Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref CIRCUITBOARD_COMMAND: Command = CommandBuilder::new("circuitboard")
        .alias("circuit")
        .alias("pcb")
        .arg(Argument::ImageBuffer)
        .public()
        .description("put an image on a circuitboard soc")
        .example(consts::Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref FLAG_COMMAND: Command = CommandBuilder::new("flag")
        .arg(Argument::ImageBuffer)
        .public()
        .description("wave an image on a flag")
        .example(consts::Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref HEART_LOCKET_COMMAND: Command = CommandBuilder::new("heartlocket")
        .arg(Argument::ImageBuffer)
        .arg(Argument::StringRemaining)
        .alias("hl")
        .public()
        .description("heart locket with a caption")
        .example("https://link.to.my/image.gif yeah")
        .usage("[image] [text]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    /* 
    pub static ref QUOTE_COMMAND: Command = CommandBuilder::new("quote")
        .arg(Argument::StringRemaining)
        .flag("white", Some(FlagKind::Text))
        .public()
        .description("quote a message")
        .example("878642522136670228")
        .usage("[message id]")
        .cooldown(Duration::from_secs(1))
        .category(CATEGORY_NAME)
        .disable()
        .build();*/
}

async fn run_makesweet_noarg_command(
    context: Arc<Context>,
    raw_image: Bytes,
    function: wsi::NoArgFunction,
) -> CommandResult {
    context.reply_with_text("processing...").await?;
    let result = function(context.assyst.clone(), raw_image, context.author_id()).await?;

    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

/* 
pub async fn run_quote_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    flags: ParsedFlags,
) -> CommandResult {
    context.reply_with_text("processing...").await?;
    let white = flags.contains_key("white");
    let guild_id = context
        .message
        .guild_id
        .ok_or_else(|| CommandError::new_boxed("This command only works in guilds"))?;

    let args = args[0].as_text();
    let raw_ids = args.split(" ");
    let mut messages = Vec::new();

    let tier = get_wsi_request_tier(&context.assyst, context.message.author.id).await?;
    let max_ids = match tier {
        0 => 1,
        1 => 5,
        2 | _ => 10,
    };

    for id in raw_ids.take(max_ids) {
        let id = id
            .parse::<u64>()
            .map_err(|_| CommandError::new_boxed(format!("`{}` is not a valid ID", id)))?;

        let message = context
            .http()
            .message(context.message.channel_id, MessageId::new(id))
            .exec()
            .await?
            .model()
            .await
            .map_err(|_| CommandError::new_boxed(format!("Failed to fetch `{}`", id)))?;

        messages.push(message);
    }

    let guild = context
        .http()
        .guild(guild_id)
        .exec()
        .await?
        .model()
        .await
        .map_err(|_| CommandError::new_boxed("Failed to fetch guild"))?;

    let bytes = annmarie::quote(&context.assyst, &messages, guild, white).await?;

    context.reply_with_image("png", bytes).await?;

    Ok(())
}*/

pub async fn run_billboard_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let raw_image = args[0].as_bytes();

    let wsi_fn = wsi::billboard;

    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes, user_id| Box::pin(wsi_fn(assyst, bytes, user_id))),
    )
    .await
}

pub async fn run_circuitboard_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let raw_image = args[0].as_bytes();

    let wsi_fn = wsi::circuitboard;

    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes, user_id| Box::pin(wsi_fn(assyst, bytes, user_id))),
    )
    .await
}

pub async fn run_flag_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let raw_image = args[0].as_bytes();

    let wsi_fn = wsi::flag;

    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes, user_id| Box::pin(wsi_fn(assyst, bytes, user_id))),
    )
    .await
}

pub async fn run_heart_locket_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let image = args[0].as_bytes();
    let text = args[1].as_text();

    context.reply_with_text("processing...").await?;
    let result =
        wsi::heart_locket(context.assyst.clone(), image, text, context.author_id()).await?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}
