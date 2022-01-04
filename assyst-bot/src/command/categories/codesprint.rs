use std::sync::Arc;

use crate::{
    command::{
        command::{Argument, Command, CommandBuilder, ParsedArgument, ParsedFlags},
        context::Context,
        registry::CommandResult,
    },
    rest::{
        self,
        codesprint::{BenchmarkResponse, Language},
    },
    util::{codeblock, download_content, nanos_to_readable},
};
use assyst_common::consts;
use lazy_static::lazy_static;
use std::fmt::Write;

const CATEGORY_NAME: &str = "misc";

lazy_static! {
    pub static ref CODESPRINT_COMMAND: Command = CommandBuilder::new("codesprint")
        .category(CATEGORY_NAME)
        .alias("cs")
        .public()
        .arg(Argument::Choice(&[
            "info", "show", "view", "best", "submit"
        ]))
        .arg(Argument::Optional(Box::new(Argument::String))) // id
        // .arg(Argument::Optional(Box::new(Argument::String))) // language for submit subcommand
        .description("Code competitions")
        .usage("codesprint")
        .example("codesprint")
        .build();
}

const SUPPORTED_LANGUAGES: &[&str] = &["Rust 1.59 (nightly)"];

async fn run_info_subcommand(
    context: Arc<Context>,
    _args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let languages = SUPPORTED_LANGUAGES
        .iter()
        .map(|x| format!("- {}", x))
        .collect::<Vec<_>>()
        .join("\n");

    let mut message = String::from(
        r#"
**Codesprint**
Compete for the fastest solutions to programming challenges.
        
**Available languages**
If you would like to have support for a language not listed, message one of the devs.
"#,
    );
    message += &languages;

    context.reply_with_text(&message).await?;
    Ok(())
}

async fn run_show_subcommand(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let id = args[1]
        .maybe_text()
        .ok_or_else(|| "No id provided")?
        .parse::<i32>()
        .map_err(|_| "Failed to parse challenge ID as an integer")?;

    let challenge = context
        .assyst
        .database
        .get_codesprint_challenge(id)
        .await?
        .ok_or_else(|| format!("Could not find challenge with id `{}`!", id))?;

    let message = format!(
        "**Challenge #{}**: {} - submitted by <@{}>\n\n{}",
        challenge.id, challenge.name, challenge.author, challenge.description
    );

    context.reply_with_text(&message).await?;
    Ok(())
}

async fn run_best_subcommand(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let id = args[1]
        .maybe_text()
        .ok_or_else(|| "No id provided")?
        .parse::<i32>()
        .map_err(|_| "Failed to parse challenge ID as an integer")?;

    let best = context.assyst.database.get_codesprint_best(id).await?;

    let mut message = format!("**Top 10 submissions for challenge #{}**\n\n", id);

    for submission in best {
        write!(
            message,
            "- <@{}>: {} [{}]\n",
            submission.author,
            nanos_to_readable(submission.mean as u32),
            submission.language
        )?;
    }

    context.reply_with_text(&message).await?;
    Ok(())
}

async fn run_submit_subcommand(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let id = args[1]
        .maybe_text()
        .ok_or_else(|| "No id provided")?
        .parse::<i32>()
        .map_err(|_| "Failed to parse challenge ID as an integer")?;

    let attachment = context.message.attachments.first().ok_or_else(|| {
        "No attachment provided. Make sure to attach the code you want to submit as an attachment."
    })?;

    let language = attachment
        .filename
        .rsplit('.')
        .next()
        .and_then(Language::from_ext)
        .ok_or_else(|| "Could not infer language from file extension. Make sure your file ends with a supported file extension.")?;

    let code = download_content(
        &context.assyst.reqwest_client,
        &attachment.url,
        consts::ABSOLUTE_INPUT_FILE_SIZE_LIMIT_BYTES,
    )
    .await
    .map(|c| String::from_utf8_lossy(&c).to_string())?;

    let response = rest::codesprint::benchmark(
        &context.assyst,
        language,
        &code,
        context.message.author.id.0,
    )
    .await?;

    match response {
        BenchmarkResponse::Success { mean, iter } => {
            let message = format!(
                "Tests passed: {}/{}\n\nTime (mean): **{}**\nIterations: **{}**",
                3,
                3,
                nanos_to_readable(mean as u32),
                iter
            );

            context
                .assyst
                .database
                .add_codesprint_submission(
                    id,
                    context.message.author.id.0 as i64,
                    mean as u32, // todo: check for overflow!
                    &code,
                    language.to_database_id(),
                )
                .await?;

            context.reply_with_text(&message).await?;
        }
        BenchmarkResponse::Error { stderr } => {
            context.reply_with_text(&codeblock(&stderr, "rs")).await?;
        }
    }
    Ok(())
}

pub async fn run_codesprint_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let subcommand = args[0].as_choice();

    match subcommand {
        "info" => run_info_subcommand(context, args, _flags).await,
        "show" | "view" => run_show_subcommand(context, args, _flags).await,
        "best" => run_best_subcommand(context, args, _flags).await,
        "submit" => run_submit_subcommand(context, args, _flags).await,
        _ => Ok(()),
    }
}
