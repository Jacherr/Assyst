use crate::{
    command::{
        command::{
            Argument, Command, CommandAvailability, CommandBuilder, CommandError, FlagKind,
            ParsedArgument, ParsedFlags,
        },
        context::Context,
        registry::CommandResult,
    },
    consts::Y21,
    rest::{
        annmarie,
        wsi::{self, ResizeMethod},
    },
    util::{bytes_to_readable, generate_list, get_wsi_request_tier},
};
use crate::{
    rest,
    util::{codeblock, get_buffer_filetype},
};
use bytes::Bytes;
use lazy_static::lazy_static;
use std::time::Duration;
use std::{borrow::Cow, sync::Arc};

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

const CATEGORY_NAME: &str = "image";

lazy_static! {
    pub static ref _3D_ROTATE_COMMAND: Command = CommandBuilder::new("3drotate")
        .alias("3d")
        .arg(Argument::ImageBuffer)
        .public()
        .description("3d rotate an image")
        .example(Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref AHSHIT_COMMAND: Command = CommandBuilder::new("ahshit")
        .arg(Argument::ImageBuffer)
        .public()
        .description("ah shit here we go again")
        .example(Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
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
        .example(Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref BLUR_COMMAND: Command = CommandBuilder::new("blur")
        .arg(Argument::ImageBuffer)
        .arg(Argument::OptionalWithDefault(
            Box::new(Argument::String),
            "3"
        ))
        .public()
        .description("blur an image")
        .example(Y21)
        .usage("[image] <power>")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref BURNTEXT_COMMAND: Command = CommandBuilder::new("burntext")
        .arg(Argument::StringRemaining)
        .public()
        .description("create burning text")
        .example("my burning text")
        .usage("[text]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref CAPTION_COMMAND: Command = CommandBuilder::new("caption")
        .arg(Argument::ImageBuffer)
        .arg(Argument::StringRemaining)
        .public()
        .description("add a caption to an image")
        .example("https://link.to.my/image.gif get real")
        .usage("[image] [caption]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref CARD_COMMAND: Command = CommandBuilder::new("card")
        .alias("discard")
        .arg(Argument::ImageBuffer)
        .public()
        .description("throw away an image on a card")
        .example(Y21)
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
        .example(Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref FISHEYE_COMMAND: Command = CommandBuilder::new("fisheye")
        .arg(Argument::ImageBuffer)
        .alias("fish")
        .public()
        .description("fisheye an image")
        .example(Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref FIX_TRANSPARENCY_COMMAND: Command = CommandBuilder::new("fixtransparency")
        .alias("ft")
        .arg(Argument::ImageBuffer)
        .public()
        .description("if a command breaks image transparency, this command may fix it")
        .example(Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref FLAG_COMMAND: Command = CommandBuilder::new("flag")
        .arg(Argument::ImageBuffer)
        .public()
        .description("wave an image on a flag")
        .example(Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    /*pub static ref FLASH_COMMAND: Command = CommandBuilder::new("flash")
        .arg(Argument::ImageBuffer)
        .public()
        .description("flash an image")
        .example(Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();*/
    pub static ref FLIP_COMMAND: Command = CommandBuilder::new("flip")
        .arg(Argument::ImageBuffer)
        .public()
        .description("flip an image")
        .example(Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref FLOP_COMMAND: Command = CommandBuilder::new("flop")
        .arg(Argument::ImageBuffer)
        .public()
        .description("flop an image")
        .example(Y21)
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
        .example(Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref FRINGE_COMMAND: Command = CommandBuilder::new("fringe")
        .arg(Argument::ImageBuffer)
        .public()
        .description("apply fringe effect to image")
        .example(Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref GHOST_COMMAND: Command = CommandBuilder::new("ghost")
        .arg(Argument::ImageBuffer)
        .arg(Argument::OptionalWithDefault(
            Box::new(Argument::String),
            "10"
        ))
        .public()
        .description("perform frame ghosting on a gif")
        .example("https://link.to.my/image.gif")
        .usage("[image] <power: 1-20>")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref GIF_LOOP_COMMAND: Command = CommandBuilder::new("gifloop")
        .arg(Argument::ImageBuffer)
        .alias("gloop")
        .public()
        .description("play a gif forward and then backward")
        .example("https://link.to.my/image.gif")
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref GIF_MAGIK_COMMAND: Command = CommandBuilder::new("gifmagik")
        .arg(Argument::ImageBuffer)
        .alias("gmagik")
        .alias("gcas")
        .alias("gifmagick")
        .public()
        .description("create a seam carved gif from the input")
        .example(Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref GIF_SCRAMBLE_COMMAND: Command = CommandBuilder::new("gifscramble")
        .arg(Argument::ImageBuffer)
        .alias("gscramble")
        .public()
        .description("scramble the frames in a gif")
        .example("https://link.to.my/image.gif")
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref GIF_SPEED_COMMAND: Command = CommandBuilder::new("gifspeed")
        .arg(Argument::ImageBuffer)
        .arg(Argument::Optional(Box::new(Argument::String)))
        .alias("gspeed")
        .public()
        .description("alter the speed of a gif (no delay argument speeds it up)")
        .example("https://link.to.my/image.gif")
        .usage("[image] <delay between frames: 2 to 100>")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref GLOBE_COMMAND: Command = CommandBuilder::new("globe")
        .arg(Argument::ImageBuffer)
        .alias("sphere")
        .public()
        .description("turn an image into a spinning globe")
        .example(Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref GRAYSCALE_COMMAND: Command = CommandBuilder::new("grayscale")
        .alias("gray")
        .arg(Argument::ImageBuffer)
        .public()
        .description("grayscale an image")
        .example(Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref IMAGE_INFO_COMMAND: Command = CommandBuilder::new("imageinfo")
        .arg(Argument::ImageBuffer)
        .alias("ii")
        .alias("exif")
        .public()
        .description("get information about an image")
        .example(Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref IMAGEMAGICK_EVAL_COMMAND: Command = CommandBuilder::new("ime")
        .arg(Argument::ImageBuffer)
        .arg(Argument::StringRemaining)
        .availability(CommandAvailability::Private)
        .description("evaluate an imagemagick script on an image")
        .example(Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref INVERT_COMMAND: Command = CommandBuilder::new("invert")
        .arg(Argument::ImageBuffer)
        .public()
        .description("invert an image")
        .example(Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref JPEG_COMMAND: Command = CommandBuilder::new("jpeg")
        .arg(Argument::ImageBuffer)
        .public()
        .description("jpegify an image")
        .example(Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref MAGIK_COMMAND: Command = CommandBuilder::new("magik")
        .arg(Argument::ImageBuffer)
        .alias("magick")
        .alias("cas")
        .alias("magic")
        .public()
        .description("perform seam carving on an image")
        .example(Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref MEME_COMMAND: Command = CommandBuilder::new("meme")
        .arg(Argument::ImageBuffer)
        .arg(Argument::StringRemaining)
        .public()
        .description("create funny meme")
        .example("312715611413413889 this is|this is an otter")
        .usage("[image] [text separated by a |]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref PRINTER_COMMAND: Command = CommandBuilder::new("printer")
        .arg(Argument::ImageBuffer)
        .public()
        .description("apply printer effect to an image")
        .example(Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref MOTIVATE_COMMAND: Command = CommandBuilder::new("motivate")
        .arg(Argument::ImageBuffer)
        .arg(Argument::StringRemaining)
        .public()
        .description("apply motivational text to an image")
        .example("https://lkink.to.my/image.png ? | will this work")
        .usage("[image] [text separated by a | marker]")
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
        .example(Y21)
        .usage("[image] <power: 1-20>")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref OCR_COMMAND: Command = CommandBuilder::new("ocr")
        .alias("read")
        .arg(Argument::ImageUrl)
        .public()
        .description("read the text on an image")
        .example(Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref OVERLAY_COMMAND: Command = CommandBuilder::new("overlay")
        .arg(Argument::ImageBuffer)
        .arg(Argument::String)
        .public()
        .description("overlay an image onto another image")
        .example("312715611413413889 finland")
        .usage("[image] [overlay]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref PAINT_COMMAND: Command = CommandBuilder::new("paint")
        .arg(Argument::ImageBuffer)
        .public()
        .description("paint an image")
        .example(Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref PIXELATE_COMMAND: Command = CommandBuilder::new("pixelate")
        .alias("pixel")
        .arg(Argument::ImageBuffer)
        .arg(Argument::Optional(Box::new(Argument::String)))
        .public()
        .description("pixelate an image")
        .example(Y21)
        .usage("[image] <pixels (smaller = more pixelated)>")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref RAINBOW_COMMAND: Command = CommandBuilder::new("rainbow")
        .arg(Argument::ImageBuffer)
        .public()
        .description("rainbowify an image")
        .example(Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref RESIZE_COMMAND: Command = CommandBuilder::new("resize")
        .flag("filter", Some(FlagKind::Choice(&["gaussian", "nearest"])))
        .arg(Argument::ImageBuffer)
        .arg(Argument::OptionalWithDefault(
            Box::new(Argument::String),
            "2"
        ))
        .public()
        .description("resize an image")
        .example(Y21)
        .example("312715611413413889 0.5")
        .example("312715611413413889 100x200")
        .usage("[image] <scale>|<widthxheight>")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref REVERSE_COMMAND: Command = CommandBuilder::new("reverse")
        .arg(Argument::ImageBuffer)
        .public()
        .description("reverse a gif")
        .example("https://link.to.my/image.gif")
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref ROTATE_COMMAND: Command = CommandBuilder::new("rotate")
        .arg(Argument::ImageBuffer)
        .arg(Argument::OptionalWithDefault(
            Box::new(Argument::String),
            "90"
        ))
        .public()
        .description("rotate an image")
        .example(Y21)
        .example("https://link.to.my/image.png 45")
        .usage("[image] <degrees>")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref SET_LOOP_COMMAND: Command = CommandBuilder::new("setloop")
        .arg(Argument::ImageBuffer)
        .arg(Argument::Choice(&["on", "off"]))
        .public()
        .description("configure whether a gif will loop")
        .example("https://link.to.my/image.gif off")
        .usage("[image] [on|off]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref SKETCH_COMMAND: Command = CommandBuilder::new("sketch")
        .arg(Argument::ImageBuffer)
        .public()
        .description("sketch an image")
        .example(Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref SPIN_COMMAND: Command = CommandBuilder::new("spin")
        .arg(Argument::ImageBuffer)
        .public()
        .description("spin an image")
        .example(Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref SPREAD_COMMAND: Command = CommandBuilder::new("spread")
        .arg(Argument::ImageBuffer)
        .public()
        .description("pixel-spread an image")
        .example(Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref SWIRL_COMMAND: Command = CommandBuilder::new("swirl")
        .arg(Argument::ImageBuffer)
        .public()
        .description("swirl an image")
        .example(Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref TEHI_COMMAND: Command = CommandBuilder::new("tehi")
        .arg(Argument::ImageBuffer)
        .public()
        .description("tehi")
        .example(Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref TERRARIA_COMMAND: Command = CommandBuilder::new("terraria")
        .arg(Argument::ImageBuffer)
        .public()
        .description("terraria music over image")
        .example(Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref WALL_COMMAND: Command = CommandBuilder::new("wall")
        .arg(Argument::ImageBuffer)
        .public()
        .description("create a wall out of an image")
        .example(Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref WAVE_COMMAND: Command = CommandBuilder::new("wave")
        .arg(Argument::ImageBuffer)
        .public()
        .description("create a wave out of an image")
        .example(Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref WORMHOLE_COMMAND: Command = CommandBuilder::new("wormhole")
        .arg(Argument::ImageBuffer)
        .public()
        .description("suck an image into a wormhole")
        .example(Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref ZOOM_COMMAND: Command = CommandBuilder::new("zoom")
        .arg(Argument::ImageBuffer)
        .public()
        .description("zoom into an image")
        .example(Y21)
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
        .example(Y21)
        .usage("[image] <power: 1-20>")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref APRIL_FOOLS_COMMAND: Command = CommandBuilder::new("aprilfools")
        .arg(Argument::ImageBuffer)
        .public()
        .description("april fools")
        .example(Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref IDENTIFY_COMMAND: Command = CommandBuilder::new("identify")
        .arg(Argument::ImageUrl)
        .public()
        .description("identify an image")
        .example("https://media.discordapp.net/attachments/827679274852286475/870284473877544980/20210729_153930.jpg")
        .usage("[url]")
        .cooldown(Duration::from_secs(1))
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
        .build();
    pub static ref RANDOMIZE_COMMAND: Command = CommandBuilder::new("randomize")
        .arg(Argument::ImageBuffer)
        .public()
        .description("sends a provided image through multiple filters")
        .example(Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(5))
        .category(CATEGORY_NAME)
        .build();
}

pub async fn run_3d_rotate_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let image = args[0].as_bytes();
    context.reply_with_text("processing...").await?;
    let result = wsi::_3d_rotate(context.assyst.clone(), image, context.author_id())
        .await
        .map_err(wsi::format_err)?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

pub async fn run_ahshit_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    run_annmarie_noarg_command!(annmarie::ahshit, args, context)
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

pub async fn run_randomize_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let mut image = Some(args[0].as_bytes());
    let mut filters = Vec::new();

    for _ in 0..3 {
        let old_image = image.take().unwrap();

        if rand::random() {
            let (route, bytes) =
                wsi::randomize(Arc::clone(&context.assyst), old_image, context.author_id())
                    .await
                    .map_err(wsi::format_err)?;

            image = Some(bytes);
            filters.push(route.strip_prefix("/").unwrap_or(route));
        } else {
            let (route, bytes) =
                annmarie::randomize(Arc::clone(&context.assyst), old_image, context.author_id())
                    .await
                    .map_err(annmarie::format_err)?;

            image = Some(bytes);
            filters.push(route.strip_prefix("/").unwrap_or(route));
        }
    }

    let image = image.unwrap();

    let filters = filters
        .into_iter()
        .map(|filter| format!("`{}`", filter))
        .collect::<Vec<_>>()
        .join(", ");

    let content = format!("filters used: {}", filters);

    let format = get_buffer_filetype(&image).unwrap_or("png");

    context
        .reply_with_image_and_text(format, image, Some(&content))
        .await
        .map(|_| ())
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
        context.assyst.clone(),
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

pub async fn run_aprilfools_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    run_annmarie_noarg_command!(annmarie::aprilfools, args, context)
}

pub async fn run_billboard_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    run_annmarie_noarg_command!(annmarie::billboard, args, context)
}

pub async fn run_blur_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let image = args[0].as_bytes();
    let power = args[1].as_text();
    context.reply_with_text("processing...").await?;
    let result = wsi::blur(context.assyst.clone(), image, context.author_id(), power)
        .await
        .map_err(wsi::format_err)?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

pub async fn run_burntext_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let text = args[0].as_text();
    context.reply_with_text("processing...").await?;
    let result = rest::burning_text(&context.assyst.clone().reqwest_client, text)
        .await
        .map_err(|e| e.to_string())?;
    context.reply_with_image("gif", result).await?;
    Ok(())
}

pub async fn run_caption_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let image = args[0].as_bytes();
    let text = args[1].as_text();
    context.reply_with_text("processing...").await?;
    let result = wsi::caption(context.assyst.clone(), image, context.author_id(), text)
        .await
        .map_err(wsi::format_err)?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
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

pub async fn run_fix_transparency_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let raw_image = args[0].as_bytes();
    let wsi_fn = wsi::fix_transparency;
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
    run_annmarie_noarg_command!(annmarie::flag, args, context)
}

/*pub async fn run_flash_command(context: Arc<Context>, args: Vec<ParsedArgument>, _flags: ParsedFlags,
) -> CommandResult {
    let raw_image = args[0].as_bytes();
    let wsi_fn = wsi::flash;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes, user_id| Box::pin(wsi_fn(assyst, bytes, user_id))),
    )
    .await
}*/

pub async fn run_flip_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let raw_image = args[0].as_bytes();
    let wsi_fn = wsi::flip;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes, user_id| Box::pin(wsi_fn(assyst, bytes, user_id))),
    )
    .await
}

pub async fn run_flop_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let raw_image = args[0].as_bytes();
    let wsi_fn = wsi::flop;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes, user_id| Box::pin(wsi_fn(assyst, bytes, user_id))),
    )
    .await
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

pub async fn run_ghost_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let image = args[0].as_bytes();
    let depth = args[1].as_text();
    context.reply_with_text("processing...").await?;
    let result = wsi::ghost(context.assyst.clone(), image, context.author_id(), depth)
        .await
        .map_err(wsi::format_err)?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

pub async fn run_gif_loop_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let raw_image = args[0].as_bytes();
    let wsi_fn = wsi::gif_loop;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes, user_id| Box::pin(wsi_fn(assyst, bytes, user_id))),
    )
    .await
}

pub async fn run_gif_magik_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let raw_image = args[0].as_bytes();
    let wsi_fn = wsi::gif_magik;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes, user_id| Box::pin(wsi_fn(assyst, bytes, user_id))),
    )
    .await
}

pub async fn run_gif_scramble_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let raw_image = args[0].as_bytes();
    let wsi_fn = wsi::gif_scramble;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes, user_id| Box::pin(wsi_fn(assyst, bytes, user_id))),
    )
    .await
}

pub async fn run_gif_speed_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let image = args[0].as_bytes();
    let delay = args[1].maybe_text();

    context.reply_with_text("processing...").await?;
    let result = wsi::gif_speed(context.assyst.clone(), image, context.author_id(), delay)
        .await
        .map_err(wsi::format_err)?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

pub async fn run_globe_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    run_annmarie_noarg_command!(annmarie::globe, args, context)
}

pub async fn run_grayscale_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let raw_image = args[0].as_bytes();
    let wsi_fn = wsi::grayscale;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes, user_id| Box::pin(wsi_fn(assyst, bytes, user_id))),
    )
    .await
}

pub async fn run_image_info_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let image = args[0].as_bytes();
    context.reply_with_text("processing...").await?;
    let result = wsi::image_info(context.assyst.clone(), image)
        .await
        .map_err(wsi::format_err)?;

    let mime_type = result.mime_type;
    let file_size = bytes_to_readable(result.file_size_bytes);
    let dimensions = format!("{}x{}", result.dimensions.0, result.dimensions.1);

    let mut table_entries: Vec<(&str, Cow<str>)> = vec![
        ("Mimetype", Cow::Borrowed(&mime_type)),
        ("File Size", Cow::Borrowed(&file_size)),
        ("Dimensions", Cow::Borrowed(&dimensions)),
        ("Colour Type", Cow::Borrowed(&result.colour_space)),
    ];

    if let Some(f) = result.frames {
        table_entries.push(("Frames", Cow::Owned(f.to_string())));
    };

    if let Some(r) = result.repeat {
        let repeat = {
            if r == -1 {
                "forever".to_owned()
            } else {
                format!("{} times", r)
            }
        };
        table_entries.push(("Repeat", Cow::Owned(repeat)));
    };

    if result.comments.len() > 0 {
        table_entries.push(("Comment", Cow::Borrowed(&result.comments[0])));
    };

    let table = generate_list("Key", "Value", &table_entries);

    context.reply_with_text(&codeblock(&table, "hs")).await?;
    Ok(())
}

pub async fn run_imagemagick_eval_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let image = args[0].as_bytes();
    let text = args[1].as_text();
    context.reply_with_text("processing...").await?;
    let result = wsi::imagemagick_eval(context.assyst.clone(), image, context.author_id(), text)
        .await
        .map_err(wsi::format_err)?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

pub async fn run_invert_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let raw_image = args[0].as_bytes();
    let wsi_fn = wsi::invert;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes, user_id| Box::pin(wsi_fn(assyst, bytes, user_id))),
    )
    .await
}

pub async fn run_jpeg_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let raw_image = args[0].as_bytes();
    let wsi_fn = wsi::jpeg;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes, user_id| Box::pin(wsi_fn(assyst, bytes, user_id))),
    )
    .await
}

pub async fn run_magik_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let raw_image = args[0].as_bytes();
    let wsi_fn = wsi::magik;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes, user_id| Box::pin(wsi_fn(assyst, bytes, user_id))),
    )
    .await
}

pub async fn run_meme_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let image = args[0].as_bytes();
    let text = args[1].as_text();

    let divider: String;

    if text.contains("|") {
        divider = "|".to_string();
    } else {
        divider = " ".to_string();
    }

    let mut parts = text.split(&divider).collect::<Vec<&str>>();
    let top_text = parts[0].to_string();
    let bottom_text = parts.drain(1..).collect::<Vec<&str>>().join(" ");

    context.reply_with_text("processing...").await?;
    let result = wsi::meme(
        context.assyst.clone(),
        image,
        context.author_id(),
        top_text.trim(),
        bottom_text.trim(),
    )
    .await
    .map_err(wsi::format_err)?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

pub async fn run_motivate_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let image = args[0].as_bytes();
    let text = args[1].as_text();

    let divider: String;

    if text.contains("|") {
        divider = "|".to_string();
    } else {
        divider = " ".to_string();
    }

    let mut parts = text.split(&divider).collect::<Vec<&str>>();
    let top_text = parts[0].to_string();
    let bottom_text = parts.drain(1..).collect::<Vec<&str>>().join(" ");

    context.reply_with_text("processing...").await?;
    let result = wsi::motivate(
        context.assyst.clone(),
        image,
        context.author_id(),
        top_text.trim(),
        bottom_text.trim(),
    )
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

pub async fn run_ocr_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let image = args[0].as_text();
    let mut result = rest::ocr_image(&context.assyst.reqwest_client, image)
        .await
        .map_err(|e| e.to_string())?;
    if result.is_empty() {
        result = "No text detected".to_owned()
    };
    context.reply_with_text(&codeblock(&result, "")).await?;
    Ok(())
}

pub async fn run_overlay_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let image = args[0].as_bytes();
    let overlay = args[1].as_text();
    context.reply_with_text("processing...").await?;
    let result = wsi::overlay(
        context.assyst.clone(),
        image,
        context.author_id(),
        &overlay.to_ascii_lowercase(),
    )
    .await
    .map_err(wsi::format_err)?;
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

pub async fn run_pixelate_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let image = args[0].as_bytes();
    let downscaled_height = args[1].maybe_text();
    context.reply_with_text("processing...").await?;
    let result = wsi::pixelate(
        context.assyst.clone(),
        image,
        context.author_id(),
        downscaled_height,
    )
    .await
    .map_err(wsi::format_err)?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

pub async fn run_printer_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let raw_image = args[0].as_bytes();
    let wsi_fn = wsi::printer;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes, user_id| Box::pin(wsi_fn(assyst, bytes, user_id))),
    )
    .await
}

pub async fn run_rainbow_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let raw_image = args[0].as_bytes();
    let wsi_fn = wsi::rainbow;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes, user_id| Box::pin(wsi_fn(assyst, bytes, user_id))),
    )
    .await
}

pub async fn run_resize_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    flags: ParsedFlags,
) -> CommandResult {
    let method = flags
        .get("filter")
        .and_then(|x| x.as_ref())
        .map(|x| x.as_text())
        .as_deref()
        .and_then(ResizeMethod::from_str)
        .unwrap_or(ResizeMethod::Nearest);

    let image = args[0].as_bytes();

    let result: Bytes;
    context.reply_with_text("processing...").await?;

    if args.get(1).is_none() {
        result = wsi::resize(context.assyst.clone(), image, context.author_id(), method)
            .await
            .map_err(wsi::format_err)?;
    } else {
        let text = args[1].as_text();
        if text.contains("x") {
            let split = text.split("x").collect::<Vec<&str>>();
            let width = split
                .get(0)
                .unwrap()
                .parse::<usize>()
                .map_err(|_| "Invalid resolution.".to_owned())?;

            let height = split
                .get(1)
                .ok_or_else(|| "Invalid resolution.".to_owned())?
                .parse::<usize>()
                .map_err(|_| "Invalid resolution.".to_owned())?;

            result = wsi::resize_width_height(
                context.assyst.clone(),
                image,
                context.author_id(),
                width,
                height,
                method,
            )
            .await
            .map_err(wsi::format_err)?;
        } else {
            let scale = text
                .parse::<f32>()
                .map_err(|_| "Invalid resolution.".to_owned())?;

            result = wsi::resize_scale(
                context.assyst.clone(),
                image,
                context.author_id(),
                scale,
                method,
            )
            .await
            .map_err(wsi::format_err)?;
        }
    }

    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

pub async fn run_reverse_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let raw_image = args[0].as_bytes();
    let wsi_fn = wsi::reverse;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes, user_id| Box::pin(wsi_fn(assyst, bytes, user_id))),
    )
    .await
}

pub async fn run_rotate_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let image = args[0].as_bytes();
    let degrees = args[1].as_text();
    context.reply_with_text("processing...").await?;
    let result = wsi::rotate(context.assyst.clone(), image, context.author_id(), degrees)
        .await
        .map_err(wsi::format_err)?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

pub async fn run_set_loop_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let image = args[0].as_bytes();
    let looping = match args[1].as_choice() {
        "on" => true,
        "off" => false,
        _ => unreachable!(),
    };
    context.reply_with_text("processing...").await?;
    let result = wsi::set_loop(context.assyst.clone(), image, context.author_id(), looping)
        .await
        .map_err(wsi::format_err)?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

pub async fn run_sketch_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    run_annmarie_noarg_command!(annmarie::sketch, args, context)
}

pub async fn run_spin_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let raw_image = args[0].as_bytes();
    let wsi_fn = wsi::spin;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes, user_id| Box::pin(wsi_fn(assyst, bytes, user_id))),
    )
    .await
}

pub async fn run_spread_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let raw_image = args[0].as_bytes();
    let wsi_fn = wsi::spread;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes, user_id| Box::pin(wsi_fn(assyst, bytes, user_id))),
    )
    .await
}

pub async fn run_swirl_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let raw_image = args[0].as_bytes();
    let wsi_fn = wsi::swirl;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes, user_id| Box::pin(wsi_fn(assyst, bytes, user_id))),
    )
    .await
}

pub async fn run_tehi_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let raw_image = args[0].as_bytes();
    let wsi_fn = wsi::tehi;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes, user_id| Box::pin(wsi_fn(assyst, bytes, user_id))),
    )
    .await
}

pub async fn run_terraria_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    run_annmarie_noarg_command!(annmarie::terraria, args, context)
}

pub async fn run_wall_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let raw_image = args[0].as_bytes();
    let wsi_fn = wsi::wall;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes, user_id| Box::pin(wsi_fn(assyst, bytes, user_id))),
    )
    .await
}

pub async fn run_wave_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let raw_image = args[0].as_bytes();
    let wsi_fn = wsi::wave;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes, user_id| Box::pin(wsi_fn(assyst, bytes, user_id))),
    )
    .await
}

pub async fn run_wormhole_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let raw_image = args[0].as_bytes();
    let wsi_fn = wsi::wormhole;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes, user_id| Box::pin(wsi_fn(assyst, bytes, user_id))),
    )
    .await
}

pub async fn run_zoom_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let raw_image = args[0].as_bytes();

    let wsi_fn = wsi::zoom;

    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes, user_id| Box::pin(wsi_fn(assyst, bytes, user_id))),
    )
    .await
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

pub async fn run_identify_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let url = args[0].as_text();
    let identify = rest::identify_image(&context.assyst.reqwest_client, url)
        .await
        .map_err(|e| e.to_string())?;

    context
        .reply_with_text(&identify)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

async fn run_wsi_noarg_command(
    context: Arc<Context>,
    raw_image: Bytes,
    function: wsi::NoArgFunction,
) -> CommandResult {
    context.reply_with_text("processing...").await?;
    let result = function(context.assyst.clone(), raw_image, context.author_id())
        .await
        .map_err(wsi::format_err)?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");

    context.reply_with_image(format, result).await?;
    Ok(())
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
