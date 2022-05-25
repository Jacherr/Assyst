use crate::{
    command::{
        command::{Argument, Command, CommandBuilder, FlagKind, ParsedArgument, ParsedFlags},
        context::Context,
        registry::CommandResult,
    },
    downloader,
    rest::{
        self,
        bt::translate_single,
        bt::{bad_translate, TranslateResult, Translation},
        wombo::{WomboResponse, WomboResponseResult, WomboStyle},
    },
    util::{codeblock, ensure_guild_manager, normalize_emojis},
};
use anyhow::{bail, Context as _};
use assyst_common::consts;
use bytes::Bytes;
use lazy_static::lazy_static;
use std::fmt::Write;
use std::{sync::Arc, time::Duration};

const CATEGORY_NAME: &str = "fun";

lazy_static! {
    pub static ref BT_COMMAND: Command = CommandBuilder::new("badtranslate")
        .alias("bt")
        .arg(Argument::StringRemaining)
        .flag("chain", None)
        .public()
        .description("badly translate text")
        .example("hello is this working")
        .example("hello is this working -chain")
        .usage("[text]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref OCRBT_COMMAND: Command = CommandBuilder::new("ocrbadtranslate")
        .alias("ocrbt")
        .arg(Argument::ImageUrl)
        .public()
        .description("OCR and then badly translate an image")
        .example("https://link.to.my/image.png")
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref OCRTR_COMMAND: Command = CommandBuilder::new("ocrtranslate")
        .alias("ocrtr")
        .alias("ocrt")
        .arg(Argument::String)
        .arg(Argument::ImageUrl)
        .public()
        .description("OCR and then translate an image")
        .example("https://link.to.my/image.png")
        .usage("[image] [language]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref RULE34_COMMAND: Command = CommandBuilder::new("rule34")
        .alias("r34")
        .arg(Argument::OptionalWithDefault(
            Box::new(Argument::StringRemaining),
            ""
        ))
        .public()
        .description("search rule34.xxx with tags")
        .example("anime")
        .usage("[tags separated by spaces]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .nsfw()
        .build();
    pub static ref COLOR_COMMAND: Command = CommandBuilder::new("color")
        .alias("colors")
        .public()
        .description("color role functionality")
        .arg(Argument::Optional(Box::new(Argument::String)))
        .arg(Argument::Optional(Box::new(Argument::String)))
        .arg(Argument::Optional(Box::new(Argument::String)))
        .example("add")
        .example("add <color name> <color code>")
        .example("add <color name> <color code>")
        .example("remove <color name>")
        .example("<color name>")
        .example("")
        .usage("red")
        .cooldown(Duration::from_secs(10))
        .category(CATEGORY_NAME)
        .build();
    pub static ref LABELS_COMMAND: Command = CommandBuilder::new("labels")
        .arg(Argument::ImageBuffer)
        .public()
        .description("create labels from image")
        .example(consts::Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref OCR_COMMAND: Command = CommandBuilder::new("ocr")
        .alias("read")
        .arg(Argument::ImageUrl)
        .public()
        .description("read the text on an image")
        .example(consts::Y21)
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref TOWAV_COMMAND: Command = CommandBuilder::new("towav")
        .alias("wavify")
        .arg(Argument::ImageBuffer)
        .public()
        .description("wavify a file")
        .example("[file]")
        .usage("[file]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref DREAM_COMMAND: Command = CommandBuilder::new("dream")
        .arg(Argument::StringRemaining)
        .flag("style", Some(FlagKind::Text))
        .public()
        .description("generates art based on a prompt")
        .example("haunted forest")
        .example("haunted forest -style psychic")
        .usage("[prompt]")
        .category(CATEGORY_NAME)
        .build();
}

pub async fn run_bt_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    flags: ParsedFlags,
) -> CommandResult {
    let text = args[0].as_text();
    let text = normalize_emojis(text);
    let wants_chain = flags.contains_key("chain");
    let TranslateResult {
        result: Translation { text, .. },
        translations,
    } = bad_translate(&context.assyst.reqwest_client, &text).await?;

    let mut output = format!("**Output**\n{}", text);

    if wants_chain {
        let _ = output.write_str("\n\n**Language chain**");

        for (index, translation) in translations.into_iter().enumerate() {
            let _ = write!(output, "\n{}) {}: ", index + 1, translation.lang);

            for (idx, ch) in translation.text.chars().enumerate() {
                if idx > consts::MAX_CHAIN_LENGTH {
                    output.push('…');
                    break;
                }

                output.push(ch);
            }
        }
    }

    context.reply_with_text(output).await?;
    Ok(())
}

pub async fn run_ocrbt_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let image = args[0].as_text();
    let result = rest::ocr_image(&context.assyst.reqwest_client, image).await?;
    if result.is_empty() {
        bail!("No text detected");
    };

    let translated = bad_translate(&context.assyst.reqwest_client, &result).await?;

    context
        .reply_with_text(codeblock(&translated.result.text, ""))
        .await?;
    Ok(())
}

pub async fn run_ocrtr_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let lang = args[0].as_text();
    let image = args[1].as_text();

    let result = rest::ocr_image(&context.assyst.reqwest_client, image).await?;
    if result.is_empty() {
        bail!("No text detected");
    };

    let translated = translate_single(&context.assyst.reqwest_client, &result, lang).await?;

    context
        .reply_with_text(codeblock(&translated.result.text, ""))
        .await?;
    Ok(())
}

pub async fn run_rule34_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let query = args[0].as_text();
    let result = rest::get_random_rule34(&context.assyst, query).await?;

    let result = result
        .first()
        .map(|first| format!("**Score: {}**\n{}", first.score, first.url))
        .unwrap_or_else(|| String::from("No results found"));

    context.reply_with_text(result).await?;
    Ok(())
}

pub async fn run_color_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _: ParsedFlags,
) -> CommandResult {
    let guild_id = context
        .message
        .guild_id
        .map(|x| x.0)
        .context("This command can only be used in servers")?;

    let mut args = args.iter();
    let ty = args.next().and_then(ParsedArgument::maybe_text);

    match ty {
        Some("add") => {
            ensure_guild_manager(&context, guild_id).await?;

            let maybe_name = args.next().and_then(ParsedArgument::maybe_text);

            if let Some(name) = maybe_name {
                let color = args
                    .next()
                    .and_then(ParsedArgument::maybe_text)
                    .map(|x| x.strip_prefix("#").unwrap_or(x))
                    .map(|x| u32::from_str_radix(x, 16))
                    .context("No color code provided")??;

                let role = context
                    .assyst
                    .http
                    .create_role(guild_id.into())
                    .name(name)
                    .color(color)
                    .await?;

                context
                    .assyst
                    .database
                    .add_color_role(role.id.0 as i64, name, guild_id as i64)
                    .await?;

                context
                    .reply_with_text("Successfully added color role")
                    .await?;
            } else {
                let guild_roles = context.assyst.http.roles(guild_id.into()).await?;

                let mut roles = Vec::new();

                for (name, color) in consts::DEFAULT_COLORS.iter() {
                    let has_color_role = guild_roles.iter().any(|x| x.name.eq(name));

                    if !has_color_role {
                        let role = context
                            .assyst
                            .http
                            .create_role(guild_id.into())
                            .name(*name)
                            .color(*color)
                            .await?;

                        roles.push((String::from(*name), role.id.0 as i64));
                    }
                }

                for role in guild_roles {
                    let is_color_role = consts::DEFAULT_COLORS
                        .iter()
                        .any(|(name, _)| role.name.eq(name));

                    if is_color_role {
                        roles.push((role.name, role.id.0 as i64));
                    }
                }

                let new_roles = roles.len();

                context
                    .assyst
                    .database
                    .bulk_add_color_roles(guild_id as i64, roles)
                    .await?;

                context
                    .reply_with_text(format!(
                        "Successfully created {} new color roles",
                        new_roles
                    ))
                    .await?;
            }
        }
        Some("remove") => {
            ensure_guild_manager(&context, guild_id).await?;

            let name = args
                .next()
                .and_then(ParsedArgument::maybe_text)
                .context("No color name provided.")?;

            let role = context
                .assyst
                .database
                .remove_color_role(guild_id as i64, name)
                .await?
                .context("Color role does not exist")?;

            context
                .assyst
                .http
                .delete_role(guild_id.into(), (role.role_id as u64).into())
                .await?;

            context.reply_with_text("Color role removed.").await?;
        }
        Some(name) => {
            let roles = context
                .assyst
                .database
                .get_color_roles(guild_id as i64)
                .await?;

            let role = roles
                .iter()
                .find(|x| x.name.eq(name))
                .context("Color role does not exist")?;

            let user_id = context.message.author.id;

            let user_roles = context
                .assyst
                .http
                .guild_member(guild_id.into(), user_id)
                .await?
                .map(|x| x.roles)
                .expect("Can't happen");

            let mut roles_without_colors = user_roles
                .iter()
                .filter(|r| roles.iter().all(|x| x.role_id as u64 != r.0))
                .copied()
                .collect::<Vec<_>>();
            roles_without_colors.push((role.role_id as u64).into());

            context
                .assyst
                .http
                .update_guild_member(guild_id.into(), user_id)
                .roles(roles_without_colors)
                .await?;

            context
                .reply_with_text(format!("Gave you the color role {}", name))
                .await?;
        }
        None => {
            let mut content = String::from("Available colors:");

            let color_roles = context
                .assyst
                .database
                .get_color_roles(guild_id as i64)
                .await?;

            let colors = color_roles
                .into_iter()
                .map(|x| x.name)
                .collect::<Vec<_>>()
                .join(", ");

            content.push_str(&codeblock(&colors, ""));
            content.push_str(&format!(
                "Use {}color <color name> to set a color",
                context.prefix
            ));

            context.reply_with_text(content).await?;
        }
    };

    Ok(())
}

pub async fn run_labels_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let image = args[0].as_bytes();
    let result = rest::annmarie::labels(context.assyst.clone(), image, context.author_id()).await?;

    let output = if result.is_empty() {
        "No text detected".to_owned()
    } else {
        let x = result
            .iter()
            .take(15)
            .map(|x| format!("{:.2}% - {}", x.score * 100.0, x.description))
            .collect::<Vec<_>>();
        x.join("\n")
    };

    context.reply_with_text(codeblock(&output, "")).await?;
    Ok(())
}

pub async fn run_ocr_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let image = args[0].as_text();
    let mut result = rest::ocr_image(&context.assyst.reqwest_client, image).await?;

    if result.is_empty() {
        result = "No text detected".to_owned()
    };

    context.reply_with_text(codeblock(&result, "")).await?;
    Ok(())
}

pub async fn run_towav_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let file = args[0].as_bytes().to_vec();

    fn conv(num: usize) -> Vec<u8> {
        [
            (num & 255) as u8,
            ((num >> 8) & 255) as u8,
            ((num >> 16) & 255) as u8,
            ((num >> 24) & 255) as u8,
        ]
        .to_vec()
    }

    const RIFF: &[u8] = "RIFF".as_bytes();
    let chunk_size = conv(file.len() + 36);
    const WAVE: &[u8] = "WAVE".as_bytes();
    const FMT_: &[u8] = "fmt ".as_bytes();
    const SUBCHUNK1_SIZE: &[u8] = &[0x10, 0, 0, 0];
    const AUDIO_FORMAT: &[u8] = &[0x1, 0];
    const NUM_CHANNELS: &[u8] = &[0x2, 0];
    const SAMPLE_RATE: &[u8] = &[0x22, 0x56, 0, 0];
    const BYTE_RATE: &[u8] = &[0x88, 0x58, 0x01, 0x00];
    const BLOCK_ALIGN: &[u8] = &[0x04, 0];
    const BITS_PER_SAMPLE: &[u8] = &[0x10, 0];
    const DATA: &[u8] = "data".as_bytes();
    let subchunk2_size = conv(file.len());

    let output = Bytes::from(
        [
            RIFF,
            &chunk_size,
            WAVE,
            FMT_,
            SUBCHUNK1_SIZE,
            AUDIO_FORMAT,
            NUM_CHANNELS,
            SAMPLE_RATE,
            BYTE_RATE,
            BLOCK_ALIGN,
            BITS_PER_SAMPLE,
            DATA,
            &subchunk2_size,
            &file,
        ]
        .concat(),
    );

    context.reply_with_file("audio/wav", output).await?;

    Ok(())
}

pub async fn run_dream_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    flags: ParsedFlags,
) -> CommandResult {
    context.reply_with_text("processing...").await?;

    let prompt = args[0].as_text();

    let style = match flags.get("style") {
        Some(Some(flag)) => flag.as_text().parse::<WomboStyle>().map_err(|_| {
            let styles = rest::wombo::STYLE_LIST.join(", ");
            anyhow::anyhow!("Invalid style. Must be one of: {styles}")
        })?,
        _ => WomboStyle::None,
    };

    let WomboResponse {
        result: WomboResponseResult { url },
    } = rest::wombo::generate(&context.assyst, style, prompt).await?;

    let file = downloader::download_content(
        &context.assyst,
        &url,
        consts::ABSOLUTE_INPUT_FILE_SIZE_LIMIT_BYTES,
    )
    .await?;

    context
        .reply_with_image("jpg", file.into())
        .await
        .map(|_| ())
}
