use crate::{
    box_str,
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
        aliases: vec![box_str!("bt")],
        args: vec![Argument::StringRemaining],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: box_str!("Badly translate a message"),
            examples: vec![box_str!("hello this is a test")],
            usage: box_str!("[text]")
        },
        name: box_str!("badtranslate"),
        cooldown_seconds: 2,
        category: "fun"
    };
    pub static ref OCRBT_COMMAND: Command = Command {
        aliases: vec![box_str!("ocrbt")],
        args: vec![Argument::ImageUrl],
        availability: CommandAvailability::Public,
        metadata: CommandMetadata {
            description: box_str!("OCR and then badly translate a message"),
            examples: vec![box_str!("https://i.jacher.io/cat.gif")],
            usage: box_str!("[text]")
        },
        name: box_str!("ocrbadtranslate"),
        cooldown_seconds: 2,
        category: "fun"
    };
}

pub async fn run_bt_command(context: Arc<Context>, args: Vec<ParsedArgument>) -> CommandResult {
    let text = force_as::text(&args[0]);
    let result = translate(&context.assyst.reqwest_client, text)
        .await
        .map_err(|e| match e {
            TranslateError::Raw(e) => e.to_string(),
            TranslateError::Reqwest(e) => e.to_string(),
        })?;
    context.reply_with_text(&result).await?;
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
        Err("No text detected")?;
    };
    let translated = translate(&context.assyst.reqwest_client, &result)
        .await
        .map_err(|e| match e {
            TranslateError::Raw(e) => e.to_string(),
            TranslateError::Reqwest(e) => e.to_string(),
        })?;
    context.reply_with_text(&codeblock(&translated, "")).await?;
    Ok(())
}
