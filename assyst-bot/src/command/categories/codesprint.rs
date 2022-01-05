use std::{sync::Arc, borrow::Cow};

use crate::{
    command::{
        command::{Argument, Command, CommandBuilder, FlagKind, ParsedArgument, ParsedFlags},
        context::Context,
        registry::CommandResult,
    },
    rest::{
        self,
        codesprint::{BenchmarkResponse, Language, Test},
    },
    util::{self, codeblock, download_content, nanos_to_readable},
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
        .flag("dry", None)
        .flag("language", Some(FlagKind::Text))
        .arg(Argument::Choice(&[
            "info", "show", "view", "best", "submit", "list"
        ]))
        .arg(Argument::Optional(Box::new(Argument::String))) // id
        .arg(Argument::Optional(Box::new(Argument::StringRemaining))) // optional codeblock for submit
        .description("Code competitions. Run `-cs info` for more information.")
        .usage("[info|show|view|best|submit|list] <[challenge id]>")
        .example("info")
        .example("show 1")
        .example("view 1")
        .example("best 1")
        .example("best 1 -language js")
        .example("submit 1 <file attachment with code>")
        .example("submit 1 -dry <file attachment with code>")
        .example("submit 1 ```js code ```")
        .example("list")
        .build();
}

const SUPPORTED_LANGUAGES: &[&str] = &["Rust 1.59 (nightly)", "JavaScript (V8 9.5)"];

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

**Rules**
This only works with some enforced rules.
Do not memoize results in globals or hardcode return values. Your code should always do all of the work.
Not following these rules can result in getting blacklisted from this command.

**JavaScript**
Your code is parsed as a function (more accurately it is parsed as `Function('input', code)`).
You can refer to `input` in your code to get the input and you need to return your result. Example:
```js
// some computation based on `input`
return input.length;
```
At the moment this does not use Node.js, so you cannot use anything from Node.js.

**Rust**
Your code needs to have a function of signature `fn run(s: &str) -> i64 {}`.
Your code is compiled with the latest nightly version, so you can use all of its features.

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
    flags: ParsedFlags,
) -> CommandResult {
    let id = args[1]
        .maybe_text()
        .ok_or_else(|| "No id provided")?
        .parse::<i32>()
        .map_err(|_| "Failed to parse challenge ID as an integer")?;

    let language = flags
        .get("language")
        .and_then(|x| x.as_ref().map(|x| x.as_text()))
        .and_then(|x| Language::from_ext(&x))
        .map(|x| x.to_database_id());

    let best = context
        .assyst
        .database
        .get_codesprint_best(id, language)
        .await?;

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
    flags: ParsedFlags,
) -> CommandResult {
    let is_dry_run = flags.contains_key("dry");

    let id = args[1]
        .maybe_text()
        .ok_or_else(|| "No id provided")?
        .parse::<i32>()
        .map_err(|_| "Failed to parse challenge ID as an integer")?;

    let tests = context
        .assyst
        .database
        .get_codesprint_tests(id)
        .await?
        .into_iter()
        .map(Test::from)
        .collect::<Vec<_>>();

    let test_count = tests.len();

    let (language, code): (Language, Cow<'_, str>) = (|| async {
        if let Some(cb) = args[2].maybe_text() {
            let (language, code) = util::parse_codeblock_with_language(cb)
                .ok_or_else(|| "Failed to parse codeblock")?;

            let language = Language::from_ext(language)
                .ok_or_else(|| "Could not infer language from file extension. Make sure your file ends with a supported file extension.")?;
            
            return Ok::<_, Box<dyn std::error::Error + Send + Sync>>((language, Cow::Borrowed(code)));
        }

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

        Ok((language, Cow::Owned(code)))
    })()
    .await?;

    let response = rest::codesprint::benchmark(
        &context.assyst,
        language,
        &code,
        context.message.author.id.0,
        tests,
    )
    .await?;

    let best_user_time = context
        .assyst
        .database
        .get_codesprint_user_fastest(
            id,
            context.message.author.id.0 as i64,
            language.to_database_id(),
        )
        .await?
        .map(|x| x.mean);

    match response {
        BenchmarkResponse::Success { mean, iter } => {
            let should_show_hint = !is_dry_run;
            let did_beat_best = best_user_time.map(|x| mean < x as f64).unwrap_or(true);

            let mut message = format!(
                "Tests passed: {}/{}\n\nTime (mean): **{}**\nIterations: **{}**\n",
                test_count,
                test_count,
                nanos_to_readable(mean as u32),
                iter
            );

            message.push_str(&format!(
                "Previous best: **{}**\n\n",
                best_user_time
                    .map(|x| nanos_to_readable(x as u32))
                    .as_deref()
                    .unwrap_or("[No time set]")
            ));

            if should_show_hint {
                message.push_str(
                    "Hint: You can test your time without submitting by appending `-dry`",
                );
            }

            if !is_dry_run && did_beat_best {
                // only store in database if this is not a dry run
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
            }

            context.reply_with_text(&message).await?;
        }
        BenchmarkResponse::InvalidStatus { stderr } => {
            context.reply_with_text(&codeblock(&stderr, "rs")).await?;
        }
        BenchmarkResponse::TestFail => {
            context.reply_with_text("Some test cases failed.").await?;
        }
    }
    Ok(())
}

async fn run_list_subcommand(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let start = args[1]
        .maybe_text()
        .unwrap_or("1")
        .parse::<i64>()
        .map_err(|_| "Failed to parse challenge ID as an integer")?;

    let challenges = context
        .assyst
        .database
        .get_codesprint_challenges(start, 10)
        .await?;

    let mut message = String::from("**Challenges**\n\n");

    for challenge in challenges {
        write!(
            message,
            "- #{} - {} - submitted by <@{}>\n",
            challenge.id, challenge.name, challenge.author
        )?;
    }

    message.push_str("\nRun `-cs show 1` to view a challenge");

    context.reply_with_text(&message).await?;

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
        "list" => run_list_subcommand(context, args, _flags).await,
        _ => Ok(()),
    }
}
