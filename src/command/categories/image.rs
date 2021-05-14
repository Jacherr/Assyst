use crate::{
    command::{
        command::{
            force_as, Argument, Command, CommandAvailability, CommandBuilder, CommandMetadata,
            ParsedArgument,
        },
        context::Context,
        registry::CommandResult,
    },
    consts::Y21,
    rest::{
        annmarie,
        wsi::{self},
    },
};
use crate::{
    rest,
    util::{codeblock, get_buffer_filetype},
};
use bytes::Bytes;
use lazy_static::lazy_static;
use std::sync::Arc;
use std::time::Duration;

macro_rules! run_annmarie_noarg_command {
    ($fn:expr, $args:expr, $context:expr) => {{
        let raw_image = force_as::image_buffer($args.drain(0..1).next().unwrap());
        let annmarie_fn = $fn;
        run_annmarie_noarg_command(
            $context,
            raw_image,
            Box::new(move |assyst, bytes| Box::pin(annmarie_fn(assyst, bytes))),
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
        .example("312715611413413889 paint")
        .usage("[image] [endoint]?[query params]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref CAPTION_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![Argument::ImageBuffer, Argument::StringRemaining],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: "add a caption to an image",
            examples: vec!["312715611413413889 yea"],
            usage: "[image] [caption]"
        },
        name: "caption",
        cooldown_seconds: 4,
        category: "image"
    };
    pub static ref CARD_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![Argument::ImageBuffer],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: "throw away an image on a card",
            examples: vec!["312715611413413889"],
            usage: "[image]"
        },
        name: "card",
        cooldown_seconds: 4,
        category: "image"
    };
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
    pub static ref FIX_TRANSPARENCY_COMMAND: Command = Command {
        aliases: vec!["ft"],
        args: vec![Argument::ImageBuffer],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description:
                "if a command breaks the transparency of a gif, use this command to fix it",
            examples: vec!["312715611413413889"],
            usage: "[image]"
        },
        name: "fixtransparency",
        cooldown_seconds: 4,
        category: "image"
    };
    pub static ref FLASH_COMMAND: Command = CommandBuilder::new("flash")
        .arg(Argument::ImageBuffer)
        .public()
        .description("flash an image")
        .example(Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
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
        .example(Y21)
        .usage("[image] <power: 1-20>")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref GIF_LOOP_COMMAND: Command = Command {
        aliases: vec!["gloop"],
        args: vec![Argument::ImageBuffer],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: "play a gif forwards then backwards",
            examples: vec!["312715611413413889"],
            usage: "[image]"
        },
        name: "gifloop",
        cooldown_seconds: 4,
        category: "image"
    };
    pub static ref GIF_MAGIK_COMMAND: Command = Command {
        aliases: vec!["gmagik", "gmagick", "gmagic", "gcas"],
        args: vec![Argument::ImageBuffer],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: "perform content aware scaling recursively on an image",
            examples: vec!["312715611413413889"],
            usage: "[image]"
        },
        name: "gifmagik",
        cooldown_seconds: 4,
        category: "image"
    };
    pub static ref GIF_SCRAMBLE_COMMAND: Command = Command {
        aliases: vec!["gscramble"],
        args: vec![Argument::ImageBuffer],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: "scramble the frames in a gif",
            examples: vec!["312715611413413889"],
            usage: "[image]"
        },
        name: "gifscramble",
        cooldown_seconds: 4,
        category: "image"
    };
    pub static ref GIF_SPEED_COMMAND: Command = Command {
        aliases: vec!["gspeed"],
        args: vec![
            Argument::ImageBuffer,
            Argument::Optional(Box::new(Argument::Integer))
        ],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: "change the speed of a gif by setting the delay between frames",
            examples: vec!["312715611413413889 2"],
            usage: "[image] <delay between frames (2 to 100)>"
        },
        name: "gifspeed",
        cooldown_seconds: 4,
        category: "image"
    };
    pub static ref GLOBE_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![Argument::ImageBuffer],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: "apply globe effect to image",
            examples: vec!["312715611413413889"],
            usage: "[image]"
        },
        name: "globe",
        cooldown_seconds: 4,
        category: "image"
    };
    pub static ref GRAYSCALE_COMMAND: Command = Command {
        aliases: vec!["gray"],
        args: vec![Argument::ImageBuffer],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: "grayscale an image",
            examples: vec!["312715611413413889"],
            usage: "[image]"
        },
        name: "grayscale",
        cooldown_seconds: 4,
        category: "image"
    };
    pub static ref IMAGEMAGICK_EVAL_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![Argument::ImageBuffer, Argument::StringRemaining],
        availability: CommandAvailability::Private,
        metadata: CommandMetadata {
            description: "evaluate an imagemagick script on an image",
            examples: vec!["312715611413413889 -reverse"],
            usage: "[image] [script]"
        },
        name: "ime",
        cooldown_seconds: 4,
        category: "image"
    };
    pub static ref INVERT_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![Argument::ImageBuffer],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: "invert an image",
            examples: vec!["312715611413413889"],
            usage: "[image]"
        },
        name: "invert",
        cooldown_seconds: 4,
        category: "image"
    };
    pub static ref JPEG_COMMAND: Command = CommandBuilder::new("jpeg")
        .arg(Argument::ImageBuffer)
        .public()
        .description("jpegify an image")
        .example(Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref MAGIK_COMMAND: Command = Command {
        aliases: vec!["magik", "magick", "magic", "cas"],
        args: vec![Argument::ImageBuffer],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: "perform content aware scaling on an image",
            examples: vec!["312715611413413889"],
            usage: "[image]"
        },
        name: "magik",
        cooldown_seconds: 4,
        category: "image"
    };
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
    pub static ref PRINTER_COMMAND: Command = Command {
        aliases: vec!["print"],
        args: vec![Argument::ImageBuffer],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: "apply printer effect to image",
            examples: vec!["312715611413413889"],
            usage: "[image]"
        },
        name: "printer",
        cooldown_seconds: 4,
        category: "image"
    };
    pub static ref MOTIVATE_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![Argument::ImageBuffer, Argument::StringRemaining],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description:
                "add motivation caption to an image, separate top and bottom text with | divider",
            examples: vec!["MOTIVATION this is funny", "HOLY SHIT | get a job"],
            usage: "[image] [text separated by a |]"
        },
        name: "motivate",
        cooldown_seconds: 4,
        category: "image"
    };
    pub static ref NEON_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![
            Argument::ImageBuffer,
            Argument::OptionalWithDefault(Box::new(Argument::String), "1")
        ],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: "apply neon effect to image",
            examples: vec!["312715611413413889"],
            usage: "[image] <radius>"
        },
        name: "neon",
        cooldown_seconds: 4,
        category: "image"
    };
    pub static ref OCR_COMMAND: Command = Command {
        aliases: vec!["read"],
        args: vec![Argument::ImageUrl],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: "read the text on an image",
            examples: vec!["312715611413413889"],
            usage: "[image]"
        },
        name: "ocr",
        cooldown_seconds: 4,
        category: "image"
    };
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
    pub static ref PAINT_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![Argument::ImageBuffer],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: "paint an image",
            examples: vec!["312715611413413889"],
            usage: "[image]"
        },
        name: "paint",
        cooldown_seconds: 4,
        category: "image"
    };
    pub static ref RAINBOW_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![Argument::ImageBuffer],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: "make an image rainbow",
            examples: vec!["312715611413413889"],
            usage: "[image]"
        },
        name: "rainbow",
        cooldown_seconds: 4,
        category: "image"
    };
    pub static ref RESIZE_COMMAND: Command = CommandBuilder::new("resize")
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
    pub static ref REVERSE_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![Argument::ImageBuffer],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: "reverse a gif",
            examples: vec!["312715611413413889"],
            usage: "[image]"
        },
        name: "reverse",
        cooldown_seconds: 4,
        category: "image"
    };
    pub static ref ROTATE_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![Argument::ImageBuffer, Argument::String],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: "rotate an image",
            examples: vec!["312715611413413889 45"],
            usage: "[image] [degrees]"
        },
        name: "rotate",
        cooldown_seconds: 4,
        category: "image"
    };
    pub static ref SET_LOOP_COMMAND: Command = Command {
        aliases: vec!["setloop"],
        args: vec![Argument::ImageBuffer, Argument::Choice(&["on", "off"])],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: "choose if you want a gif to loop or not",
            examples: vec!["312715611413413889"],
            usage: "[image]"
        },
        name: "setlooping",
        cooldown_seconds: 4,
        category: "image"
    };
    pub static ref SKETCH_COMMAND: Command = CommandBuilder::new("sketch")
        .arg(Argument::ImageBuffer)
        .public()
        .description("sketch an image")
        .example(Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref SPIN_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![Argument::ImageBuffer],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: "spin an image",
            examples: vec!["312715611413413889"],
            usage: "[image]"
        },
        name: "spin",
        cooldown_seconds: 4,
        category: "image"
    };
    pub static ref SPREAD_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![Argument::ImageBuffer],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: "pixel-spread an image",
            examples: vec!["312715611413413889"],
            usage: "[image]"
        },
        name: "spread",
        cooldown_seconds: 4,
        category: "image"
    };
    pub static ref SWIRL_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![Argument::ImageBuffer],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: "swirl an image",
            examples: vec!["312715611413413889"],
            usage: "[image]"
        },
        name: "swirl",
        cooldown_seconds: 4,
        category: "image"
    };
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
    pub static ref WALL_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![Argument::ImageBuffer],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: "create a wall out of an image",
            examples: vec!["312715611413413889"],
            usage: "[image]"
        },
        name: "wall",
        cooldown_seconds: 4,
        category: "image"
    };
    pub static ref WAVE_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![Argument::ImageBuffer],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: "create a wave out of an image",
            examples: vec!["312715611413413889"],
            usage: "[image]"
        },
        name: "wave",
        cooldown_seconds: 4,
        category: "image"
    };
    pub static ref WORMHOLE_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![Argument::ImageBuffer],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: "suck an image into a wormhole",
            examples: vec!["312715611413413889"],
            usage: "[image]"
        },
        name: "wormhole",
        cooldown_seconds: 4,
        category: "image"
    };
    pub static ref ZOOM_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![Argument::ImageBuffer],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: "zoom into an image",
            examples: vec!["312715611413413889"],
            usage: "[image]"
        },
        name: "zoom",
        cooldown_seconds: 4,
        category: "image"
    };
    pub static ref ZOOM_BLUR_COMMAND: Command = Command {
        aliases: vec!["zb"],
        args: vec![
            Argument::ImageBuffer,
            Argument::OptionalWithDefault(Box::new(Argument::String), "1")
        ],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: "apply zoom blur effect to image",
            examples: vec!["312715611413413889"],
            usage: "[image] <power>"
        },
        name: "zoomblur",
        cooldown_seconds: 4,
        category: "image"
    };
    pub static ref APRIL_FOOLS_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![Argument::ImageBuffer],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: "april fools",
            examples: vec!["312715611413413889"],
            usage: "[image]"
        },
        name: "aprilfools",
        cooldown_seconds: 4,
        category: "image"
    };
}

pub async fn run_3d_rotate_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    context.reply_with_text("processing...").await?;
    let result = wsi::_3d_rotate(context.assyst.clone(), image)
        .await
        .map_err(wsi::format_err)?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

pub async fn run_ahshit_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    run_annmarie_noarg_command!(annmarie::ahshit, args, context)
}

pub async fn run_annmarie_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let endpoint = force_as::text(&args[0]);
    context.reply_with_text("processing...").await?;
    let result = annmarie::request_bytes(
        context.assyst.clone(),
        &format!("/{}", endpoint),
        image,
        &[],
    )
    .await
    .map_err(annmarie::format_err)?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

pub async fn run_aprilfools_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    run_annmarie_noarg_command!(annmarie::aprilfools, args, context)
}

pub async fn run_caption_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let text = force_as::text(&args[0]);
    context.reply_with_text("processing...").await?;
    let result = wsi::caption(context.assyst.clone(), image, text)
        .await
        .map_err(wsi::format_err)?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

pub async fn run_card_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    run_annmarie_noarg_command!(annmarie::card, args, context)
}

pub async fn run_fisheye_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    run_annmarie_noarg_command!(annmarie::fisheye, args, context)
}

pub async fn run_fix_transparency_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let wsi_fn = wsi::fix_transparency;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes| Box::pin(wsi_fn(assyst, bytes))),
    )
    .await
}

pub async fn run_flash_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let wsi_fn = wsi::flash;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes| Box::pin(wsi_fn(assyst, bytes))),
    )
    .await
}

pub async fn run_flip_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let wsi_fn = wsi::flip;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes| Box::pin(wsi_fn(assyst, bytes))),
    )
    .await
}

pub async fn run_flop_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let wsi_fn = wsi::flop;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes| Box::pin(wsi_fn(assyst, bytes))),
    )
    .await
}

pub async fn run_f_shift_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    run_annmarie_noarg_command!(annmarie::f_shift, args, context)
}

pub async fn run_fringe_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    run_annmarie_noarg_command!(annmarie::fringe, args, context)
}

pub async fn run_ghost_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let depth = force_as::text(&args[0]);
    context.reply_with_text("processing...").await?;
    let result = wsi::ghost(context.assyst.clone(), image, depth)
        .await
        .map_err(wsi::format_err)?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

pub async fn run_gif_loop_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let wsi_fn = wsi::gif_loop;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes| Box::pin(wsi_fn(assyst, bytes))),
    )
    .await
}

pub async fn run_gif_magik_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let wsi_fn = wsi::gif_magik;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes| Box::pin(wsi_fn(assyst, bytes))),
    )
    .await
}

pub async fn run_gif_scramble_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let wsi_fn = wsi::gif_scramble;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes| Box::pin(wsi_fn(assyst, bytes))),
    )
    .await
}

pub async fn run_gif_speed_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let delay = if args[0].is_nothing() {
        None
    } else {
        Some(force_as::text(&args[0]))
    };
    context.reply_with_text("processing...").await?;
    let result = wsi::gif_speed(context.assyst.clone(), image, delay)
        .await
        .map_err(wsi::format_err)?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

pub async fn run_globe_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    run_annmarie_noarg_command!(annmarie::globe, args, context)
}

pub async fn run_grayscale_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let wsi_fn = wsi::grayscale;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes| Box::pin(wsi_fn(assyst, bytes))),
    )
    .await
}

pub async fn run_imagemagick_eval_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let text = force_as::text(&args[0]);
    context.reply_with_text("processing...").await?;
    let result = wsi::imagemagick_eval(context.assyst.clone(), image, text)
        .await
        .map_err(wsi::format_err)?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

pub async fn run_invert_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let wsi_fn = wsi::invert;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes| Box::pin(wsi_fn(assyst, bytes))),
    )
    .await
}

pub async fn run_jpeg_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let wsi_fn = wsi::jpeg;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes| Box::pin(wsi_fn(assyst, bytes))),
    )
    .await
}

pub async fn run_magik_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let wsi_fn = wsi::magik;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes| Box::pin(wsi_fn(assyst, bytes))),
    )
    .await
}

pub async fn run_meme_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let text = force_as::text(&args[0]);

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
    let result = wsi::meme(context.assyst.clone(), image, &top_text, &bottom_text)
        .await
        .map_err(wsi::format_err)?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

pub async fn run_motivate_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let text = force_as::text(&args[0]);

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
    let result = wsi::motivate(context.assyst.clone(), image, &top_text, &bottom_text)
        .await
        .map_err(wsi::format_err)?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

pub async fn run_neon_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let radius = force_as::text(&args[0]);
    context.reply_with_text("processing...").await?;
    let result = annmarie::neon(context.assyst.clone(), image, radius)
        .await
        .map_err(annmarie::format_err)?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

pub async fn run_ocr_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let arg = args.drain(0..1).next().unwrap();
    let image = force_as::text(&arg);
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
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let overlay = force_as::text(&args[0]);
    context.reply_with_text("processing...").await?;
    let result = wsi::overlay(context.assyst.clone(), image, &overlay.to_ascii_lowercase())
        .await
        .map_err(wsi::format_err)?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

pub async fn run_paint_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    run_annmarie_noarg_command!(annmarie::paint, args, context)
}

pub async fn run_printer_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let wsi_fn = wsi::printer;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes| Box::pin(wsi_fn(assyst, bytes))),
    )
    .await
}

pub async fn run_rainbow_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let wsi_fn = wsi::rainbow;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes| Box::pin(wsi_fn(assyst, bytes))),
    )
    .await
}

pub async fn run_resize_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let image = force_as::image_buffer(args.drain(0..1).next().unwrap());

    let result: Bytes;
    context.reply_with_text("processing...").await?;

    if args.get(0).is_none() {
        result = wsi::resize(context.assyst.clone(), image)
            .await
            .map_err(wsi::format_err)?;
    } else {
        let text = force_as::text(args.get(0).unwrap());
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

            result = wsi::resize_width_height(context.assyst.clone(), image, width, height)
                .await
                .map_err(wsi::format_err)?;
        } else {
            let scale = text
                .parse::<f32>()
                .map_err(|_| "Invalid resolution.".to_owned())?;

            result = wsi::resize_scale(context.assyst.clone(), image, scale)
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
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let wsi_fn = wsi::reverse;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes| Box::pin(wsi_fn(assyst, bytes))),
    )
    .await
}

pub async fn run_rotate_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let degrees = force_as::text(&args[0]);
    context.reply_with_text("processing...").await?;
    let result = wsi::rotate(context.assyst.clone(), image, degrees)
        .await
        .map_err(wsi::format_err)?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

pub async fn run_set_loop_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let looping = match force_as::choice(&args[0]) {
        "on" => true,
        "off" => false,
        _ => unreachable!(),
    };
    context.reply_with_text("processing...").await?;
    let result = wsi::set_loop(context.assyst.clone(), image, looping)
        .await
        .map_err(wsi::format_err)?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

pub async fn run_sketch_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    run_annmarie_noarg_command!(annmarie::sketch, args, context)
}

pub async fn run_spin_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let wsi_fn = wsi::spin;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes| Box::pin(wsi_fn(assyst, bytes))),
    )
    .await
}

pub async fn run_spread_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let wsi_fn = wsi::spread;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes| Box::pin(wsi_fn(assyst, bytes))),
    )
    .await
}

pub async fn run_swirl_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let wsi_fn = wsi::swirl;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes| Box::pin(wsi_fn(assyst, bytes))),
    )
    .await
}

pub async fn run_tehi_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let wsi_fn = wsi::tehi;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes| Box::pin(wsi_fn(assyst, bytes))),
    )
    .await
}

pub async fn run_terraria_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    run_annmarie_noarg_command!(annmarie::terraria, args, context)
}

pub async fn run_wall_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let wsi_fn = wsi::wall;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes| Box::pin(wsi_fn(assyst, bytes))),
    )
    .await
}

pub async fn run_wave_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let wsi_fn = wsi::wave;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes| Box::pin(wsi_fn(assyst, bytes))),
    )
    .await
}

pub async fn run_wormhole_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let wsi_fn = wsi::wormhole;
    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes| Box::pin(wsi_fn(assyst, bytes))),
    )
    .await
}

pub async fn run_zoom_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());

    let wsi_fn = wsi::zoom;

    run_wsi_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes| Box::pin(wsi_fn(assyst, bytes))),
    )
    .await
}

pub async fn run_zoom_blur_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let power = force_as::text(&args[0]);
    context.reply_with_text("processing...").await?;
    let result = annmarie::zoom_blur(context.assyst.clone(), image, power)
        .await
        .map_err(annmarie::format_err)?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

async fn run_wsi_noarg_command(
    context: Arc<Context>,
    raw_image: Bytes,
    function: wsi::NoArgFunction,
) -> CommandResult {
    context.reply_with_text("processing...").await?;
    let result = function(context.assyst.clone(), raw_image)
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
    let result = function(context.assyst.clone(), raw_image)
        .await
        .map_err(annmarie::format_err)?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}
