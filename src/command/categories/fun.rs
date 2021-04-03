use crate::{
    command::{
        command::{
            force_as, Argument, Command, CommandAvailability, CommandMetadata, ParsedArgument,
        },
        context::Context,
        registry::CommandResult,
    },
    rest::{
        self,
        bt::{translate, TranslateError},
    },
    util::codeblock,
};
use lazy_static::lazy_static;
use std::sync::Arc;

lazy_static! {
    pub static ref BT_COMMAND: Command = Command {
        aliases: vec!["bt"],
        args: vec![Argument::StringRemaining],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: "Badly translate a message",
            examples: vec!["hello this is a test"],
            usage: "[text]"
        },
        name: "badtranslate",
        cooldown_seconds: 2,
        category: "fun"
    };
    pub static ref BTDEBUG_COMMAND: Command = Command {
        aliases: vec![],
        args: vec![Argument::StringRemaining],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: "Badly translate a message and returns debug information",
            examples: vec!["hello this is a test"],
            usage: "[text]"
        },
        name: "btdebug",
        cooldown_seconds: 2,
        category: "fun"
    };
    pub static ref OCRBT_COMMAND: Command = Command {
        aliases: vec!["ocrbt"],
        args: vec![Argument::ImageUrl],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: "OCR and then badly translate a message",
            examples: vec!["https://i.jacher.io/cat.gif"],
            usage: "[text]"
        },
        name: "ocrbadtranslate",
        cooldown_seconds: 2,
        category: "fun"
    };
}

pub async fn run_bt_command(context: Arc<Context>, args: Vec<ParsedArgument>) -> CommandResult {
    let text = force_as::text(&args[0]);
    let translated = translate(&context.assyst.reqwest_client, text)
        .await
        .map_err(|e| match e {
            TranslateError::Raw(e) => e.to_string(),
            TranslateError::Reqwest(e) => e.to_string(),
        })?;

    let output = format!("**Output**\n{}", translated.result.text);
    context.reply_with_text(&output).await?;
    Ok(())
}

pub async fn run_btdebug_command(context: Arc<Context>, args: Vec<ParsedArgument>) -> CommandResult {
    let text = force_as::text(&args[0]);
    let translated = translate(&context.assyst.reqwest_client, text)
        .await
        .map_err(|e| match e {
            TranslateError::Raw(e) => e.to_string(),
            TranslateError::Reqwest(e) => e.to_string(),
        })?;

    let chain = translated.translations.iter()
        .enumerate()
        .map(|(index, translation)| format!("{}) {}: {}\n", index + 1, translation.lang, translation.text))
        .collect::<String>();

    let output = format!("**Output**\n{}\n\n**Language Chain**\n{}", translated.result.text, chain);
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

    let translated = translate(&context.assyst.reqwest_client, &result)
        .await
        .map_err(|e| match e {
            TranslateError::Raw(e) => e.to_string(),
            TranslateError::Reqwest(e) => e.to_string(),
        })?;

    context.reply_with_text(&codeblock(&translated.result.text, "")).await?;
    Ok(())
}
