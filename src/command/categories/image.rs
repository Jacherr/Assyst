use crate::{
    box_str,
    command::{
        command::{
            force_as, Argument, Command, CommandAvailability, CommandMetadata, ParsedArgument,
        },
        context::Context,
        registry::CommandResult,
    },
    consts::WORKING_FILESIZE_LIMIT_BYTES,
    rest::wsi,
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
            description: box_str!("3d rotate an image"),
            examples: vec![box_str!("312715611413413889")],
            usage: box_str!("[image]")
        },
        name: box_str!("3drotate"),
        cooldown_seconds: 4,
        category: "image"
    };
    pub static ref CAPTION_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![Argument::ImageBuffer, Argument::StringRemaining],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: box_str!("add a caption to an image"),
            examples: vec![box_str!("312715611413413889 yea")],
            usage: box_str!("[image] [caption]")
        },
        name: box_str!("caption"),
        cooldown_seconds: 4,
        category: "image"
    };
    pub static ref GIF_SCRAMBLE_COMMAND: Command = Command {
        aliases: vec![box_str!("gscramble")],
        args: vec![Argument::ImageBuffer],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: box_str!("scramble the frames in a gif"),
            examples: vec![box_str!("312715611413413889")],
            usage: box_str!("[image]")
        },
        name: box_str!("gifscramble"),
        cooldown_seconds: 4,
        category: "image"
    };
    pub static ref GIF_SPEED_COMMAND: Command = Command {
        aliases: vec![box_str!("gspeed")],
        args: vec![Argument::ImageBuffer, Argument::OptionalWithDefault(Box::new(Argument::StringRemaining), "2")],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: box_str!("change speed of gif"),
            examples: vec![box_str!("312715611413413889 2")],
            usage: box_str!("[image] <delay>")
        },
        name: box_str!("gifspeed"),
        cooldown_seconds: 4,
        category: "image"
    };
    pub static ref IMAGEMAGICK_EVAL_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![Argument::ImageBuffer, Argument::StringRemaining],
        availability: CommandAvailability::Private,
        metadata: CommandMetadata {
            description: box_str!("evaluate an imagemagick script on an image"),
            examples: vec![box_str!("312715611413413889 -reverse")],
            usage: box_str!("[image] [script]")
        },
        name: box_str!("ime"),
        cooldown_seconds: 4,
        category: "image"
    };
    pub static ref MOTIVATE_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![Argument::ImageBuffer, Argument::StringRemaining],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: box_str!(
                "add motivation caption to an image, separate top and bottom text with | divider"
            ),
            examples: vec![
                box_str!("MOTIVATION this is funny"),
                box_str!("HOLY SHIT | get a job")
            ],
            usage: box_str!("[image] [text separated by a |]")
        },
        name: box_str!("motivate"),
        cooldown_seconds: 4,
        category: "image"
    };
    pub static ref OCR_COMMAND: Command = Command {
        aliases: vec![box_str!("read")],
        args: vec![Argument::ImageUrl],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: box_str!("read the text on an image"),
            examples: vec![box_str!("312715611413413889")],
            usage: box_str!("[image]")
        },
        name: box_str!("ocr"),
        cooldown_seconds: 4,
        category: "image"
    };
    pub static ref RAINBOW_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![Argument::ImageBuffer],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: box_str!("make an image rainbow"),
            examples: vec![box_str!("312715611413413889")],
            usage: box_str!("[image]")
        },
        name: box_str!("rainbow"),
        cooldown_seconds: 4,
        category: "image"
    };
    pub static ref REVERSE_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![Argument::ImageBuffer],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: box_str!("reverse a gif"),
            examples: vec![box_str!("312715611413413889")],
            usage: box_str!("[image]")
        },
        name: box_str!("reverse"),
        cooldown_seconds: 4,
        category: "image"
    };
    pub static ref ROTATE_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![Argument::ImageBuffer, Argument::String],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: box_str!("rotate an image"),
            examples: vec![box_str!("312715611413413889 45")],
            usage: box_str!("[image] [degrees]")
        },
        name: box_str!("rotate"),
        cooldown_seconds: 4,
        category: "image"
    };
    pub static ref SPIN_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![Argument::ImageBuffer],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: box_str!("spin an image"),
            examples: vec![box_str!("312715611413413889")],
            usage: box_str!("[image]")
        },
        name: box_str!("spin"),
        cooldown_seconds: 4,
        category: "image"
    };
    pub static ref SPREAD_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![Argument::ImageBuffer],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: box_str!("pixel-spread an image"),
            examples: vec![box_str!("312715611413413889")],
            usage: box_str!("[image]")
        },
        name: box_str!("spread"),
        cooldown_seconds: 4,
        category: "image"
    };
    pub static ref WALL_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![Argument::ImageBuffer],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: box_str!("create a wall out of an image"),
            examples: vec![box_str!("312715611413413889")],
            usage: box_str!("[image]")
        },
        name: box_str!("wall"),
        cooldown_seconds: 4,
        category: "image"
    };
    pub static ref WAVE_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![Argument::ImageBuffer],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: box_str!("create a wave out of an image"),
            examples: vec![box_str!("312715611413413889")],
            usage: box_str!("[image]")
        },
        name: box_str!("wave"),
        cooldown_seconds: 4,
        category: "image"
    };
    pub static ref WORMHOLE_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![Argument::ImageBuffer],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: box_str!("suck an image into a wormhole"),
            examples: vec![box_str!("312715611413413889")],
            usage: box_str!("[image]")
        },
        name: box_str!("wormhole"),
        cooldown_seconds: 4,
        category: "image"
    };
    pub static ref ZOOM_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![Argument::ImageBuffer],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: box_str!("zoom into an image"),
            examples: vec![box_str!("312715611413413889")],
            usage: box_str!("[image]")
        },
        name: box_str!("zoom"),
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

pub async fn run_gif_scramble_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let image = compress_if_large(context.clone(), raw_image).await?;
    context.reply_with_text("processing...").await?;
    let result = wsi::gif_scramble(context.assyst.clone(), image)
        .await
        .map_err(wsi::format_err)?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
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

pub async fn run_rainbow_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let image = compress_if_large(context.clone(), raw_image).await?;
    context.reply_with_text("processing...").await?;
    let result = wsi::rainbow(context.assyst.clone(), image)
        .await
        .map_err(wsi::format_err)?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

pub async fn run_reverse_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let image = compress_if_large(context.clone(), raw_image).await?;
    context.reply_with_text("processing...").await?;
    let result = wsi::reverse(context.assyst.clone(), image)
        .await
        .map_err(wsi::format_err)?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
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

pub async fn run_spin_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let image = compress_if_large(context.clone(), raw_image).await?;
    context.reply_with_text("processing...").await?;
    let result = wsi::spin(context.assyst.clone(), image)
        .await
        .map_err(wsi::format_err)?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

pub async fn run_spread_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let image = compress_if_large(context.clone(), raw_image).await?;
    context.reply_with_text("processing...").await?;
    let result = wsi::spread(context.assyst.clone(), image)
        .await
        .map_err(wsi::format_err)?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

pub async fn run_wall_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let image = compress_if_large(context.clone(), raw_image).await?;
    context.reply_with_text("processing...").await?;
    let result = wsi::wall(context.assyst.clone(), image)
        .await
        .map_err(wsi::format_err)?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

pub async fn run_wave_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let image = compress_if_large(context.clone(), raw_image).await?;
    context.reply_with_text("processing...").await?;
    let result = wsi::wave(context.assyst.clone(), image)
        .await
        .map_err(wsi::format_err)?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

pub async fn run_wormhole_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let image = compress_if_large(context.clone(), raw_image).await?;
    context.reply_with_text("processing...").await?;
    let result = wsi::wormhole(context.assyst.clone(), image)
        .await
        .map_err(wsi::format_err)?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}

pub async fn run_zoom_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let raw_image = force_as::image_buffer(args.drain(0..1).next().unwrap());
    let image = compress_if_large(context.clone(), raw_image).await?;
    context.reply_with_text("processing...").await?;
    let result = wsi::zoom(context.assyst.clone(), image)
        .await
        .map_err(wsi::format_err)?;
    let format = get_buffer_filetype(&result).unwrap_or_else(|| "png");
    context.reply_with_image(format, result).await?;
    Ok(())
}
