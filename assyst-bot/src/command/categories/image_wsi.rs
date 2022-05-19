use crate::{
    command::{
        command::{
            Argument, Command, CommandAvailability, CommandBuilder, FlagKind, ParsedArgument,
            ParsedFlags,
        },
        context::Context,
        registry::CommandResult,
    },
    rest::wsi,
    util::{bytes_to_readable, generate_list},
};
use crate::{
    rest,
    util::{codeblock, get_buffer_filetype},
};
use anyhow::{anyhow, Context as _};
use assyst_common::consts;
use bytes::Bytes;
use lazy_static::lazy_static;
use shared::query_params::ResizeMethod;
use std::time::Duration;
use std::{borrow::Cow, sync::Arc};

const CATEGORY_NAME: &str = "image (wsi)";

lazy_static! {
    pub static ref _3D_ROTATE_COMMAND: Command = CommandBuilder::new("3drotate")
        .alias("3d")
        .arg(Argument::ImageBuffer)
        .public()
        .description("3d rotate an image")
        .example(consts::Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref AHSHIT_COMMAND: Command = CommandBuilder::new("ahshit")
        .arg(Argument::ImageBuffer)
        .public()
        .description("ah shit here we go again")
        .example(consts::Y21)
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
        .example(consts::Y21)
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
    pub static ref DRIP_COMMAND: Command = CommandBuilder::new("drip")
        .arg(Argument::ImageBuffer)
        .public()
        .description("among us drip music over image")
        .example(consts::Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref FIX_TRANSPARENCY_COMMAND: Command = CommandBuilder::new("fixtransparency")
        .alias("ft")
        .arg(Argument::ImageBuffer)
        .public()
        .description("if a command breaks image transparency, this command may fix it")
        .example(consts::Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    /*pub static ref FLASH_COMMAND: Command = CommandBuilder::new("flash")
        .arg(Argument::ImageBuffer)
        .public()
        .description("flash an image")
        .example(consts::Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();*/
    pub static ref FLIP_COMMAND: Command = CommandBuilder::new("flip")
        .arg(Argument::ImageBuffer)
        .public()
        .description("flip an image")
        .example(consts::Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref FLOP_COMMAND: Command = CommandBuilder::new("flop")
        .arg(Argument::ImageBuffer)
        .public()
        .description("flop an image")
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
    pub static ref FRAMES_COMMAND: Command = CommandBuilder::new("frames")
        .arg(Argument::ImageBuffer)
        .public()
        .description("get frames of a gif")
        .example(consts::Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref FEMURBREAKER_COMMAND: Command = CommandBuilder::new("femurbreaker")
        .arg(Argument::ImageBuffer)
        .public()
        .description("femurbreaker over image")
        .example(consts::Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref GHOST_COMMAND: Command = CommandBuilder::new("ghost")
        .arg(Argument::ImageBuffer)
        .arg(Argument::OptionalWithDefault(
            Box::new(Argument::Integer),
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
        .example(consts::Y21)
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
        .arg(Argument::Optional(Box::new(Argument::Integer)))
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
        .example(consts::Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref GRAYSCALE_COMMAND: Command = CommandBuilder::new("grayscale")
        .alias("gray")
        .arg(Argument::ImageBuffer)
        .public()
        .description("grayscale an image")
        .example(consts::Y21)
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
        .example(consts::Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref IMAGEMAGICK_EVAL_COMMAND: Command = CommandBuilder::new("ime")
        .arg(Argument::ImageBuffer)
        .arg(Argument::StringRemaining)
        .availability(CommandAvailability::Private)
        .description("evaluate an imagemagick script on an image")
        .example(consts::Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref INVERT_COMMAND: Command = CommandBuilder::new("invert")
        .arg(Argument::ImageBuffer)
        .public()
        .description("invert an image")
        .example(consts::Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref JPEG_COMMAND: Command = CommandBuilder::new("jpeg")
        .arg(Argument::ImageBuffer)
        .public()
        .description("jpegify an image")
        .example(consts::Y21)
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
        .example(consts::Y21)
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
    pub static ref NEON_COMMAND: Command = CommandBuilder::new("neon")
        .arg(Argument::ImageBuffer)
        .arg(Argument::OptionalWithDefault(
            Box::new(Argument::Integer),
            "1"
        ))
        .public()
        .description("neon an image")
        .example(consts::Y21)
        .usage("[image] <power>")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref PRINTER_COMMAND: Command = CommandBuilder::new("printer")
        .arg(Argument::ImageBuffer)
        .public()
        .description("apply printer effect to an image")
        .example(consts::Y21)
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
        .example(consts::Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref PIXELATE_COMMAND: Command = CommandBuilder::new("pixelate")
        .alias("pixel")
        .arg(Argument::ImageBuffer)
        .arg(Argument::Optional(Box::new(Argument::Integer)))
        .public()
        .description("pixelate an image")
        .example(consts::Y21)
        .usage("[image] <pixels (smaller = more pixelated)>")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref RAINBOW_COMMAND: Command = CommandBuilder::new("rainbow")
        .arg(Argument::ImageBuffer)
        .public()
        .description("rainbowify an image")
        .example(consts::Y21)
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
        .example(consts::Y21)
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
        .example(consts::Y21)
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
    pub static ref SIREN_COMMAND: Command = CommandBuilder::new("siren")
        .arg(Argument::ImageBuffer)
        .public()
        .description("siren over image")
        .example(consts::Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref SPIN_COMMAND: Command = CommandBuilder::new("spin")
        .arg(Argument::ImageBuffer)
        .public()
        .description("spin an image")
        .example(consts::Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref SPREAD_COMMAND: Command = CommandBuilder::new("spread")
        .arg(Argument::ImageBuffer)
        .public()
        .description("pixel-spread an image")
        .example(consts::Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref SWIRL_COMMAND: Command = CommandBuilder::new("swirl")
        .arg(Argument::ImageBuffer)
        .public()
        .description("swirl an image")
        .example(consts::Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref TEHI_COMMAND: Command = CommandBuilder::new("tehi")
        .arg(Argument::ImageBuffer)
        .public()
        .description("tehi")
        .example(consts::Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref UNCAPTION_COMMAND: Command = CommandBuilder::new("uncaption")
        .arg(Argument::ImageBuffer)
        .public()
        .description("Remove a caption from an image")
        .example(consts::Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref SWEDEN_COMMAND: Command = CommandBuilder::new("sweden")
        .alias("minecraft")
        .arg(Argument::ImageBuffer)
        .public()
        .description("minecraft music over image")
        .example(consts::Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref TERRARIA_COMMAND: Command = CommandBuilder::new("terraria")
        .arg(Argument::ImageBuffer)
        .public()
        .description("terraria music over image")
        .example(consts::Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref WALL_COMMAND: Command = CommandBuilder::new("wall")
        .arg(Argument::ImageBuffer)
        .public()
        .description("create a wall out of an image")
        .example(consts::Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref WAVE_COMMAND: Command = CommandBuilder::new("wave")
        .arg(Argument::ImageBuffer)
        .public()
        .description("create a wave out of an image")
        .example(consts::Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref WORMHOLE_COMMAND: Command = CommandBuilder::new("wormhole")
        .arg(Argument::ImageBuffer)
        .public()
        .description("suck an image into a wormhole")
        .example(consts::Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref ZOOM_COMMAND: Command = CommandBuilder::new("zoom")
        .arg(Argument::ImageBuffer)
        .public()
        .description("zoom into an image")
        .example(consts::Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref APRIL_FOOLS_COMMAND: Command = CommandBuilder::new("aprilfools")
        .arg(Argument::ImageBuffer)
        .public()
        .description("april fools")
        .example(consts::Y21)
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
    pub static ref RANDOMIZE_COMMAND: Command = CommandBuilder::new("randomize")
        .alias("badcommand")
        .arg(Argument::ImageBuffer)
        .flag("exclude", Some(FlagKind::List))
        .public()
        .description("sends a provided image through multiple random filters")
        .example(consts::Y21)
        .example(format!(r#"{} -exclude "card jpeg""#, consts::Y21))
        .usage(r#"[image] -exclude "command1 command2""#)
        .cooldown(Duration::from_secs(5))
        .category(CATEGORY_NAME)
        .disable()
        .build();
    pub static ref ZOOM_BLUR_COMMAND: Command = CommandBuilder::new("zoomblur")
        .alias("zb")
        .arg(Argument::ImageBuffer)
        .arg(Argument::OptionalWithDefault(
            Box::new(Argument::Decimal),
            "2"
        ))
        .public()
        .description("apply zoomblur effect to image")
        .example(consts::Y21)
        .example(format!("{} 2.5", consts::Y21))
        .usage("[image] <power: 1-20>")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref SOFTGLOW_COMMAND: Command = CommandBuilder::new("bloom")
        .alias("softglow")
        .arg(Argument::ImageBuffer)
        .flag("radius", Some(FlagKind::Number))
        .flag("brightness", Some(FlagKind::Number))
        .flag("sharpness", Some(FlagKind::Number))
        .public()
        .description("bloom an image")
        .example(consts::Y21)
        .example(format!("{} -radius 5", consts::Y21))
        .example(format!("{} -brightness 30", consts::Y21))
        .example(format!("{} -sharpness 85", consts::Y21))
        .usage("[image] <-radius: number> <-brightness: number> <-sharpness: number>")
        .cooldown(Duration::from_secs(4))
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
    let result = wsi::_3d_rotate(context.assyst.clone(), image, context.author_id()).await?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

pub async fn run_ahshit_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let raw_image = args[0].as_bytes();
    let wsi_fn = wsi::ahshit;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes, user_id| Box::pin(wsi_fn(assyst, bytes, user_id))),
    )
    .await
}

pub async fn run_randomize_command(
    _context: Arc<Context>,
    _args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    /*
    async fn inner_randomize(
        assyst: &Assyst,
        image: Bytes,
        user_id: UserId,
        wsi_routes: &mut Vec<&'static str>,
        annmarie_routes: &mut Vec<&'static str>,
    ) -> (
        &'static str,
        Result<Bytes, Box<dyn std::error::Error + Send + Sync>>,
    ) {
        let can_use_annmarie = annmarie_routes.len() > 0;
        if rand::random() || !can_use_annmarie {
            let (route, bytes) = wsi::randomize(assyst, image, user_id, wsi_routes).await;
            let filter = route.strip_prefix("/").unwrap_or(route);

            match bytes {
                Ok(bytes) => (filter, Ok(bytes)),
                Err(e) => (filter, Err(wsi::format_err(e).into())),
            }
        } else {
            let (route, bytes) = annmarie::randomize(assyst, image, user_id, annmarie_routes).await;
            let filter = route.strip_prefix("/").unwrap_or(route);

            match bytes {
                Ok(bytes) => (filter, Ok(bytes)),
                Err(e) => (filter, Err(annmarie::format_err(e).into())),
            }
        }
    }

    let mut image = Some(args[0].as_bytes());
    let mut filters = Vec::new();

    let exclude = flags
        .remove("exclude")
        .flatten()
        .and_then(ParsedFlagKind::into_list)
        .unwrap_or_else(Vec::new);

    let mut wsi_exclude = Vec::new();
    let mut annmarie_exclude = Vec::new();

    for filter in exclude {
        if let Some(filter) = wsi::routes::command_name_to_route(&filter) {
            wsi_exclude.push(filter);
        } else if let Some(filter) = annmarie::routes::command_name_to_route(&filter) {
            annmarie_exclude.push(filter);
        }
    }

    let mut wsi_routes = wsi::routes::RANDOMIZABLE_ROUTES
        .iter()
        .filter(|route| !wsi_exclude.contains(route))
        .copied()
        .collect::<Vec<_>>();

    let mut annmarie_routes = annmarie::routes::RANDOMIZABLE_ROUTES
        .iter()
        .filter(|route| !annmarie_exclude.contains(route))
        .copied()
        .collect::<Vec<_>>();

    let available_filters = wsi_routes.len() + annmarie_routes.len();
    if available_filters < consts::RANDOMIZE_COUNT {
        return Err(
            "Not enough filters to choose from! Perhaps you are excluding too many commands?"
                .into(),
        );
    }

    let mut maybe_error = None;

    for _ in 0..consts::RANDOMIZE_COUNT {
        let old_image = image.take().unwrap();

        let resp = inner_randomize(
            &context.assyst,
            old_image.clone(),
            context.author_id(),
            &mut wsi_routes,
            &mut annmarie_routes,
        )
        .await;

        match resp {
            (filter, Ok(bytes)) => {
                filters.push(filter);
                image = Some(bytes);
            }
            (filter, Err(e)) => {
                maybe_error = Some((filter, e));
                image = Some(old_image);
                break;
            }
        };
    }

    let image = image.unwrap();

    let filters = filters
        .into_iter()
        .map(|filter| format!("`{}`", filter))
        .collect::<Vec<_>>()
        .join(", ");

    let mut content = format!("filters used: {}", filters);

    if let Some((filter, error)) = maybe_error {
        content += &format!(
            "\n\n:warning: An error occured while running filter `{}`: {}",
            filter, error
        );
    }

    let format = get_buffer_filetype(&image).unwrap_or("png");

    context
        .reply_with_image_and_text(&format!("image/{}", format), image, Some(content))
        .await
        .map(|_| ())
        */
    todo!()
}

pub async fn run_aprilfools_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let raw_image = args[0].as_bytes();
    let wsi_fn = wsi::aprilfools;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes, user_id| Box::pin(wsi_fn(assyst, bytes, user_id))),
    )
    .await
}

pub async fn run_blur_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let image = args[0].as_bytes();
    let power = args[1].as_text();
    context.reply_with_text("processing...").await?;
    let result = wsi::blur(context.assyst.clone(), image, context.author_id(), power).await?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

pub async fn run_bloom_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    flags: ParsedFlags,
) -> CommandResult {
    let image = args[0].as_bytes();
    let radius = flags
        .get("radius")
        .and_then(|x| x.as_ref())
        .map(|x| x.as_text())
        .unwrap_or(Cow::Borrowed("5"))
        .to_string();

    let brightness = flags
        .get("brightness")
        .and_then(|x| x.as_ref())
        .map(|x| x.as_text())
        .unwrap_or(Cow::Borrowed("35"))
        .to_string();

    let sharpness = flags
        .get("sharpness")
        .and_then(|x| x.as_ref())
        .map(|x| x.as_text())
        .unwrap_or(Cow::Borrowed("85"))
        .to_string();

    let radius = radius.parse::<usize>().unwrap();
    let brightness = brightness.parse::<usize>().unwrap();
    let sharpness = sharpness.parse::<usize>().unwrap();

    context.reply_with_text("processing...").await?;
    let result = wsi::bloom(
        context.assyst.clone(),
        image,
        context.author_id(),
        radius,
        brightness,
        sharpness,
    )
    .await?;
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
    let result = rest::burning_text(text).await?;
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
    let result = wsi::caption(context.assyst.clone(), image, context.author_id(), text).await?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

pub async fn run_drip_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let image = args[0].as_bytes();
    context.reply_with_text("processing...").await?;
    let result = wsi::audio(context.assyst.clone(), image, "drip", context.author_id()).await?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
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

pub async fn run_femurbreaker_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let image = args[0].as_bytes();
    context.reply_with_text("processing...").await?;
    let result = wsi::audio(
        context.assyst.clone(),
        image,
        "femurbreaker",
        context.author_id(),
    )
    .await?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

pub async fn run_f_shift_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let raw_image = args[0].as_bytes();
    let wsi_fn = wsi::frame_shift;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes, user_id| Box::pin(wsi_fn(assyst, bytes, user_id))),
    )
    .await
}

pub async fn run_fisheye_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let raw_image = args[0].as_bytes();
    let wsi_fn = wsi::fisheye;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes, user_id| Box::pin(wsi_fn(assyst, bytes, user_id))),
    )
    .await
}

pub async fn run_frames_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let image = args[0].as_bytes();
    context.reply_with_text("processing...").await?;
    let result = wsi::frames(context.assyst.clone(), image, context.author_id()).await?;
    context.reply_with_file("application/zip", result).await?;
    Ok(())
}

pub async fn run_ghost_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let image = args[0].as_bytes();
    let depth = args[1].as_text();
    context.reply_with_text("processing...").await?;
    let result = wsi::ghost(context.assyst.clone(), image, context.author_id(), depth).await?;
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
    let result = wsi::gif_speed(context.assyst.clone(), image, context.author_id(), delay).await?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
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
    let result = wsi::image_info(context.assyst.clone(), image).await?;

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

    context.reply_with_text(codeblock(&table, "hs")).await?;
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
    let result =
        wsi::imagemagick_eval(context.assyst.clone(), image, context.author_id(), text).await?;
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

pub async fn run_globe_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let raw_image = args[0].as_bytes();
    let wsi_fn = wsi::globe;
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
    .await?;
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
    .await?;
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
    let radius = args[1].as_text().parse::<usize>().unwrap_or(1).clamp(1, 20);
    context.reply_with_text("processing...").await?;
    let result = wsi::neon(context.assyst.clone(), image, context.author_id(), radius).await?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
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
    .await?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

pub async fn run_paint_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let raw_image = args[0].as_bytes();
    let wsi_fn = wsi::paint;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes, user_id| Box::pin(wsi_fn(assyst, bytes, user_id))),
    )
    .await
}

pub async fn run_pixelate_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let image = args[0].as_bytes();
    let downscaled_height = args[1]
        .maybe_text()
        .map(|s| s.parse::<usize>().unwrap_or(usize::MAX));

    context.reply_with_text("processing...").await?;
    let result = wsi::pixelate(
        context.assyst.clone(),
        image,
        context.author_id(),
        downscaled_height,
    )
    .await?;
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
        result = wsi::resize(context.assyst.clone(), image, context.author_id(), method).await?;
    } else {
        let text = args[1].as_text();
        if text.contains("x") {
            let split = text.split("x").collect::<Vec<&str>>();
            let width = split
                .get(0)
                .unwrap()
                .parse::<usize>()
                .map_err(|_| anyhow!("Invalid resolution."))?;

            let height = split
                .get(1)
                .context("Invalid resolution.")?
                .parse::<usize>()
                .map_err(|_| anyhow!("Invalid resolution."))?;

            result = wsi::resize_width_height(
                context.assyst.clone(),
                image,
                context.author_id(),
                width,
                height,
                method,
            )
            .await?;
        } else {
            let scale = text
                .parse::<f32>()
                .map_err(|_| anyhow!("Invalid resolution"))?;

            result = wsi::resize_scale(
                context.assyst.clone(),
                image,
                context.author_id(),
                scale,
                method,
            )
            .await?;
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
    let result = wsi::rotate(context.assyst.clone(), image, context.author_id(), degrees).await?;
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
    let result = wsi::set_loop(context.assyst.clone(), image, context.author_id(), looping).await?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

pub async fn run_siren_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let image = args[0].as_bytes();
    context.reply_with_text("processing...").await?;
    let result = wsi::audio(context.assyst.clone(), image, "siren", context.author_id()).await?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
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

pub async fn run_uncaption_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let raw_image = args[0].as_bytes();
    let wsi_fn = wsi::uncaption;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes, user_id| Box::pin(wsi_fn(assyst, bytes, user_id))),
    )
    .await
}

pub async fn run_sweden_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let image = args[0].as_bytes();
    context.reply_with_text("processing...").await?;
    let result = wsi::audio(context.assyst.clone(), image, "sweden", context.author_id()).await?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

pub async fn run_terraria_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let image = args[0].as_bytes();
    context.reply_with_text("processing...").await?;
    let result = wsi::audio(
        context.assyst.clone(),
        image,
        "terraria",
        context.author_id(),
    )
    .await?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
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
    let factor = args[1].as_text();
    context.reply_with_text("processing...").await?;
    let result = wsi::zoom_blur(
        context.assyst.clone(),
        image,
        context.author_id(),
        factor.parse::<f64>().unwrap(),
    )
    .await?;
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
    let identify = rest::identify::identify_image(
        &context.assyst.reqwest_client,
        url,
        context.assyst.config.auth.rapidapi.as_ref(),
    )
    .await?;

    let caption = identify
        .description
        .as_ref()
        .and_then(|description| description.captions.get(0))
        .map(|caption| {
            format!(
                "I think it's {} ({}% confidence)",
                caption.text,
                (caption.confidence * 100f32) as u8
            )
        })
        .unwrap_or_else(|| String::from(consts::IDENTIFY_ERROR_MESSAGE));

    context.reply_with_text(caption).await?;
    Ok(())
}

async fn run_wsi_noarg_command(
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
