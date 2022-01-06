use crate::util::get_buffer_filetype;
use crate::{
    command::{
        command::{
            Argument, Command, CommandAvailability, CommandBuilder, CommandError, FlagKind,
            ParsedArgument, ParsedFlags,
        },
        context::Context,
        registry::CommandResult,
    },
    rest::{annmarie, wsi},
    util::get_wsi_request_tier,
};
use assyst_common::consts;
use bytes::Bytes;
use lazy_static::lazy_static;
use std::sync::Arc;
use std::time::Duration;

const CATEGORY_NAME: &str = "image (annmarie)";

macro_rules! run_annmarie_noarg_command {
    ($fn:expr, $args:expr, $context:expr) => {{
        let raw_image = $args[0].as_bytes();
        let annmarie_fn = $fn;
        run_annmarie_noarg_command(
            $context,
            raw_image,
            Box::new(move |assyst, bytes, user_id| Box::pin(annmarie_fn(assyst, bytes, user_id))),
        )
        .await
    }};
}

lazy_static! {
    pub static ref ANNMARIE_COMMAND: Command = CommandBuilder::new("annmarie")
        .alias("ann")
        .arg(Argument::ImageBuffer)
        .arg(Argument::StringRemaining)
        .availability(CommandAvailability::Private)
        .description("run annmarie command on endpoint")
        .example("https://link.to.my/image.gif paint")
        .usage("[image] [endoint]?[query params]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref BILLBOARD_COMMAND: Command = CommandBuilder::new("billboard")
        .arg(Argument::ImageBuffer)
        .public()
        .description("display an image on a billboard")
        .example(consts::Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref CARD_COMMAND: Command = CommandBuilder::new("card")
        .alias("discard")
        .arg(Argument::ImageBuffer)
        .public()
        .description("throw away an image on a card")
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
    pub static ref FISHEYE_COMMAND: Command = CommandBuilder::new("fisheye")
        .arg(Argument::ImageBuffer)
        .alias("fish")
        .public()
        .description("fisheye an image")
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
    pub static ref F_SHIFT_COMMAND: Command = CommandBuilder::new("frameshift")
        .arg(Argument::ImageBuffer)
        .alias("butt")
        .alias("fshift")
        .public()
        .description("frameshift an image")
        .example(consts::Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .disable()
        .build();
    pub static ref FRINGE_COMMAND: Command = CommandBuilder::new("fringe")
        .arg(Argument::ImageBuffer)
        .public()
        .description("apply fringe effect to image")
        .example(consts::Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .disable()
        .build();
    pub static ref GLOBE_COMMAND: Command = CommandBuilder::new("globe")
        .arg(Argument::ImageBuffer)
        .alias("sphere")
        .public()
        .description("turn an image into a spinning globe")
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
    pub static ref NEON_COMMAND: Command = CommandBuilder::new("neon")
        .arg(Argument::ImageBuffer)
        .arg(Argument::OptionalWithDefault(
            Box::new(Argument::String),
            "1"
        ))
        .public()
        .description("neon an image")
        .example(consts::Y21)
        .usage("[image] <power: 1-20>")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref PAINT_COMMAND: Command = CommandBuilder::new("paint")
        .arg(Argument::ImageBuffer)
        .public()
        .description("paint an image")
        .example(consts::Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref SKETCH_COMMAND: Command = CommandBuilder::new("sketch")
        .arg(Argument::ImageBuffer)
        .public()
        .description("sketch an image")
        .example(consts::Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref ZOOM_BLUR_COMMAND: Command = CommandBuilder::new("zoomblur")
        .alias("zb")
        .arg(Argument::ImageBuffer)
        .arg(Argument::OptionalWithDefault(
            Box::new(Argument::String),
            "2"
        ))
        .public()
        .description("apply zoomblur effect to image")
        .example(consts::Y21)
        .usage("[image] <power: 1-20>")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
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
        .build();
}

async fn run_annmarie_noarg_command(
    context: Arc<Context>,
    raw_image: Bytes,
    function: annmarie::NoArgFunction,
) -> CommandResult {
    context.reply_with_text("processing...").await?;
    let result = function(context.assyst.clone(), raw_image, context.author_id())
        .await
        .map_err(annmarie::format_err)?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

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
            .message(context.message.channel_id, id.into())
            .await
            .map_err(|_| CommandError::new_boxed(format!("Failed to fetch `{}`", id)))?
            .ok_or_else(|| CommandError::new_boxed("Message not found"))?;

        messages.push(message);
    }

    let guild = context
        .http()
        .guild(guild_id)
        .await?
        .ok_or_else(|| CommandError::new_boxed("Failed to fetch guild"))?;

    let bytes = annmarie::quote(&context.assyst, &messages, guild, white)
        .await
        .map_err(annmarie::format_err)?;

    context.reply_with_image("png", bytes).await?;

    Ok(())
}

pub async fn run_annmarie_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let image = args[0].as_bytes();
    let endpoint = args[1].as_text();
    context.reply_with_text("processing...").await?;
    let result = annmarie::request_bytes(
        &context.assyst,
        &format!("/{}", endpoint),
        image,
        &[],
        context.author_id(),
    )
    .await
    .map_err(annmarie::format_err)?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

pub async fn run_billboard_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    run_annmarie_noarg_command!(annmarie::billboard, args, context)
}

pub async fn run_card_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    run_annmarie_noarg_command!(annmarie::card, args, context)
}

pub async fn run_circuitboard_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    run_annmarie_noarg_command!(annmarie::circuitboard, args, context)
}

pub async fn run_fisheye_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    run_annmarie_noarg_command!(annmarie::fisheye, args, context)
}

pub async fn run_flag_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    run_annmarie_noarg_command!(annmarie::flag, args, context)
}

pub async fn run_f_shift_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    run_annmarie_noarg_command!(annmarie::f_shift, args, context)
}

pub async fn run_fringe_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    run_annmarie_noarg_command!(annmarie::fringe, args, context)
}

pub async fn run_globe_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    run_annmarie_noarg_command!(annmarie::globe, args, context)
}

pub async fn run_heart_locket_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let image = args[0].as_bytes();
    let text = args[1].as_text();

    context.reply_with_text("processing...").await?;
    let result = wsi::heart_locket(context.assyst.clone(), image, text, context.author_id())
        .await
        .map_err(wsi::format_err)?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

pub async fn run_neon_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let image = args[0].as_bytes();
    let radius = args[1].as_text();
    context.reply_with_text("processing...").await?;
    let result = annmarie::neon(context.assyst.clone(), image, context.author_id(), radius)
        .await
        .map_err(annmarie::format_err)?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

pub async fn run_paint_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    run_annmarie_noarg_command!(annmarie::paint, args, context)
}

pub async fn run_sketch_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    run_annmarie_noarg_command!(annmarie::sketch, args, context)
}

pub async fn run_zoom_blur_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let image = args[0].as_bytes();
    let power = args[1].as_text();
    context.reply_with_text("processing...").await?;
    let result = annmarie::zoom_blur(context.assyst.clone(), image, context.author_id(), power)
        .await
        .map_err(annmarie::format_err)?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}
