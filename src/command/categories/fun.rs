use crate::{
    command::{
        command::{
            force_as, Argument, Command, CommandBuilder,
            ParsedArgument,
        },
        context::Context,
        registry::CommandResult,
    },
    consts,
    rest::{self, bt::bad_translate, bt::translate_single},
    util::codeblock,
};
use futures::TryFutureExt;
use lazy_static::lazy_static;
use std::{sync::Arc, time::Duration};

const CATEGORY_NAME: &str = "fun";

lazy_static! {
    pub static ref BT_COMMAND: Command = CommandBuilder::new("badtranslate")
        .alias("bt")
        .arg(Argument::StringRemaining)
        .public()
        .description("badly translate text")
        .example("hello is this working")
        .usage("[text]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref BTDEBUG_COMMAND: Command = CommandBuilder::new("btdebug")
        .arg(Argument::StringRemaining)
        .public()
        .description("badly translate text with debug info")
        .example("hello is this working")
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
        .arg(Argument::ImageUrl)
        .public()
        .description("OCR and then translate an image")
        .example("https://link.to.my/image.png")
        .usage("[image]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
    pub static ref RULE34_COMMAND: Command = CommandBuilder::new("rule34")
        .alias("r34")
        .arg(Argument::ImageUrl)
        .public()
        .description("search rule34.xxx")
        .example("anime")
        .usage("[query]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
        .build();
}

pub async fn run_bt_command(context: Arc<Context>, args: Vec<ParsedArgument>) -> CommandResult {
    let text = force_as::text(&args[0]);
    let translated = bad_translate(&context.assyst.reqwest_client, text)
        .await
        .map_err(|e| e.to_string())?;

    let output = format!("**Output**\n{}", translated.result.text);
    context.reply_with_text(&output).await?;
    Ok(())
}

pub async fn run_btdebug_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
) -> CommandResult {
    let text = force_as::text(&args[0]);
    let translated = bad_translate(&context.assyst.reqwest_client, text)
        .await
        .map_err(|e| e.to_string())?;

    let chain = translated
        .translations
        .iter()
        .enumerate()
        .map(|(index, translation)| {
            let output = format!(
                "{}) {}: {}\n",
                index + 1,
                translation.lang,
                translation.text
            );

            let suffix = if output.len() > consts::MAX_CHAIN_LENGTH {
                "…\n"
            } else {
                "\n"
            };

            output
                .chars()
                .take(consts::MAX_CHAIN_LENGTH)
                .collect::<String>()
                + suffix
        })
        .collect::<String>();

    let output = format!(
        "**Output**\n{}\n\n**Language Chain**\n{}",
        translated.result.text, chain
    );
    context.reply_with_text(&output).await?;
    Ok(())
}

pub async fn run_ocrbt_command(
    context: Arc<Context>,
    mut args: Vec<ParsedArgument>,
) -> CommandResult {
    let arg = args.drain(0..1).next().unwrap();
    let image = force_as::text(&arg);
    let result = rest::ocr_image(&context.assyst.reqwest_client, image)
        .await
        .map_err(|e| e.to_string())?;
    if result.is_empty() {
        return Err("No text detected".into());
    };

    let translated = bad_translate(&context.assyst.reqwest_client, &result)
        .await
        .map_err(|e| e.to_string())?;

    context
        .reply_with_text(&codeblock(&translated.result.text, ""))
        .await?;
    Ok(())
}

pub async fn run_ocrtr_command(context: Arc<Context>, args: Vec<ParsedArgument>) -> CommandResult {
    let lang = force_as::text(&args[0]);
    let image = force_as::text(&args[1]);

    let result = rest::ocr_image(&context.assyst.reqwest_client, image)
        .await
        .map_err(|e| e.to_string())?;
    if result.is_empty() {
        return Err("No text detected".into());
    };

    let translated = translate_single(&context.assyst.reqwest_client, &result, lang)
        .await
        .map_err(|e| e.to_string())?;

    context
        .reply_with_text(&codeblock(&translated.result.text, ""))
        .await?;
    Ok(())
}

pub async fn run_rule34_command(context: Arc<Context>, _: Vec<ParsedArgument>) -> CommandResult {
    tokio::time::sleep(Duration::from_millis(1500)).await;

    context.reply_err("450 Blocked By Windows Parental Controls")
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}
