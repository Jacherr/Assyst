use crate::{
    command::{
        command::{
            force_as, Argument, Command, CommandAvailability, CommandMetadata, ParsedArgument,
        },
        context::Context,
        registry::CommandResult,
    },
    consts::WORKING_FILESIZE_LIMIT_BYTES,
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

lazy_static! {
    pub static ref _3D_ROTATE_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![Argument::ImageBuffer],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: "3d rotate an image",
            examples: vec!["312715611413413889"],
            usage: "[image]"
        },
        name: "3drotate",
        cooldown_seconds: 4,
        category: "image"
    };
    pub static ref ANNMARIE_COMMAND: Command = Command {
        aliases: vec!["ann"],
        args: vec![Argument::ImageBuffer, Argument::StringRemaining],
        availability: CommandAvailability::Private,
        metadata: CommandMetadata {
            description: "run an image command from an annmarie endpoint",
            examples: vec!["312715611413413889 neon"],
            usage: "[image] [endpoint]"
        },
        name: "annmarie",
        cooldown_seconds: 4,
        category: "image"
    };
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

pub async fn run_3d_rotate_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let image = compress_if_large(context.clone(), raw_image).await?;
    context.reply_with_text("processing...").await?;
    let result = wsi::_3d_rotate(context.assyst.clone(), image)
        .await
        .map_err(wsi::format_err)?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

pub async fn run_annmarie_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let image = compress_if_large(context.clone(), raw_image).await?;
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

pub async fn run_caption_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let image = compress_if_large(context.clone(), raw_image).await?;
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
    let image = compress_if_large(context.clone(), raw_image).await?;
    context.reply_with_text("processing...").await?;
    let result = annmarie::card(context.assyst.clone(), image)
        .await
        .map_err(annmarie::format_err)?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
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
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let image = compress_if_large(context.clone(), raw_image).await?;
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
    let image = compress_if_large(context.clone(), raw_image).await?;
    context.reply_with_text("processing...").await?;
    let result = annmarie::globe(context.assyst.clone(), image)
        .await
        .map_err(annmarie::format_err)?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
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
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let image = compress_if_large(context.clone(), raw_image).await?;
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
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let image = compress_if_large(context.clone(), raw_image).await?;
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
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let image = compress_if_large(context.clone(), raw_image).await?;
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
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let image = compress_if_large(context.clone(), raw_image).await?;
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
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let image = compress_if_large(context.clone(), raw_image).await?;
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
