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
            Argument::OptionalWithDefault(Box::new(Argument::Integer), "2")
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
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let annmarie_fn = annmarie::ahshit;
    run_annmarie_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes| Box::pin(annmarie_fn(assyst, bytes))),
    )
    .await
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
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let annmarie_fn = annmarie::aprilfools;
    run_annmarie_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes| Box::pin(annmarie_fn(assyst, bytes))),
    )
    .await
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
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let annmarie_fn = annmarie::card;
    run_annmarie_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes| Box::pin(annmarie_fn(assyst, bytes))),
    )
    .await
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
    let delay = force_as::text(&args[0]);
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
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let annmarie_fn = annmarie::globe;
    run_annmarie_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes| Box::pin(annmarie_fn(assyst, bytes))),
    )
    .await
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

pub async fn run_paint_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let annmarie_fn = annmarie::paint;
    run_annmarie_noarg_command(
        context,
        raw_image,
        Box::new(move |assyst, bytes| Box::pin(annmarie_fn(assyst, bytes))),
    )
    .await
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
