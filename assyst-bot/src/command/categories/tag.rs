use std::{convert::TryInto, sync::Arc};

use anyhow::{ensure, Context as _};
use lazy_static::lazy_static;
use std::fmt::Write;

use crate::{
    command::{
        command::{Argument, Command, CommandBuilder, ParsedArgument, ParsedFlags},
        context::Context,
        registry::CommandResult,
    },
    util,
};

const CATEGORY_NAME: &str = "misc";
const DEFAULT_LIST_COUNT: i64 = 10;
const DESCRIPTION: &str = r#"
-t <name>                    :: Look up a tag by its name and respond with its contents
-t create <name> <content>   :: Create a tag with the given name and content
-t delete <name>             :: Delete a tag by its name
-t edit <name> <content>     :: Edit a tag by its name and new content
-t list [<page, default=0>]  :: List tags created in this guild
-t info <name>               :: Get information about a tag"#;

lazy_static! {
    pub static ref TAG_COMMAND: Command = CommandBuilder::new("tag")
        .category(CATEGORY_NAME)
        .alias("t")
        .description(DESCRIPTION)
        .public()
        .arg(Argument::String)
        .arg(Argument::Optional(Box::new(Argument::String)))
        .arg(Argument::Optional(Box::new(Argument::StringRemaining)))
        .build();
}

async fn run_create_subcommand(context: Arc<Context>, args: Vec<ParsedArgument>) -> CommandResult {
    let author = context.message.author.id.0;
    let guild_id = context.message.guild_id.unwrap().0;
    let name = args
        .get(1)
        .and_then(|t| t.maybe_text())
        .context("No tag name provided.")?;

    ensure!(name.len() < 20, "tag name must be less than 20 characters");

    let content = args
        .get(2)
        .and_then(|t| t.maybe_text())
        .context("No tag contents provided.")?;

    let success = context
        .assyst
        .database
        .add_tag(author.try_into()?, guild_id.try_into()?, name, content)
        .await?;

    ensure!(success, "Tag already exists in this guild.");

    context
        .reply_with_text(format!("Successfully created tag `{}`.", name))
        .await?;

    Ok(())
}

async fn run_delete_subcommand(context: Arc<Context>, args: Vec<ParsedArgument>) -> CommandResult {
    let author = context.message.author.id.0;
    let guild_id = context.message.guild_id.unwrap().0;
    let name = args
        .get(1)
        .and_then(|t| t.maybe_text())
        .context("No tag name provided.")?;

    let success = context
        .assyst
        .database
        .remove_tag(author.try_into()?, guild_id.try_into()?, name)
        .await?;

    ensure!(
        success,
        "Failed to delete tag. Does it exist, and do you own it?"
    );

    context
        .reply_with_text(format!("Successfully deleted tag `{}`.", name))
        .await?;

    Ok(())
}

async fn run_edit_subcommand(context: Arc<Context>, args: Vec<ParsedArgument>) -> CommandResult {
    let author = context.message.author.id.0;
    let guild_id = context.message.guild_id.unwrap().0;
    let name = args
        .get(1)
        .and_then(|t| t.maybe_text())
        .context("No tag name provided.")?;
    let data = args
        .get(2)
        .and_then(|t| t.maybe_text())
        .context("No tag contents provided.")?;

    let success = context
        .assyst
        .database
        .edit_tag(author.try_into()?, guild_id.try_into()?, name, data)
        .await?;

    ensure!(
        success,
        "Failed to edit tag. Does it exist, and do you own it?"
    );

    context
        .reply_with_text(format!("Successfully edited tag `{}`.", name))
        .await?;

    Ok(())
}

async fn run_list_subcommand(context: Arc<Context>, args: Vec<ParsedArgument>) -> CommandResult {
    let guild_id = context.message.guild_id.unwrap().0;
    let page = args
        .get(1)
        .and_then(|t| t.maybe_text())
        .map(|t| t.parse::<i64>())
        .unwrap_or(Ok(1))?;

    ensure!(page >= 1, "page must be greater or equal to 1");

    let offset = (page - 1) * DEFAULT_LIST_COUNT;

    let tags = context
        .assyst
        .database
        .get_tags_paged(guild_id.try_into()?, offset, DEFAULT_LIST_COUNT)
        .await?;

    let mut message = format!(
        "🗒️ **Tags in this server**\nView a tag by running `{0}t <name>`, or go to the next page by running `{0}t list {1}`\n\n",
        context.prefix,
        page + 1
    );

    for (index, tag) in tags.into_iter().enumerate() {
        let offset = (index as i64) + offset + 1;
        write!(message, "{}. {} (<@{}>)\n", offset, tag.name, tag.author)?;
    }

    write!(
        message,
        "\nShowing {} tags (page {})",
        DEFAULT_LIST_COUNT, page
    )?;

    context.reply_with_text(message).await?;

    Ok(())
}

async fn run_info_subcommand(context: Arc<Context>, args: Vec<ParsedArgument>) -> CommandResult {
    let guild_id = context.message.guild_id.unwrap().0;
    let name = args
        .get(1)
        .map(|t| t.as_text())
        .context("No tag name provided.")?;

    let tag = context
        .assyst
        .database
        .get_tag(guild_id.try_into()?, name)
        .await?
        .context("No tag found.")?;

    let fmt = util::format_discord_timestamp(tag.created_at as u64);
    let message = format!(
        "🗒️ **Tag information: **{}\n\nAuthor: <@{}>\nCreated: {}",
        tag.name, tag.author, fmt
    );

    context.reply_with_text(message).await?;

    Ok(())
}

async fn run_tag_subcommand(context: Arc<Context>, args: Vec<ParsedArgument>) -> CommandResult {
    let guild_id = context.message.guild_id.unwrap().0;
    let name = args
        .get(0)
        .map(|t| t.as_text())
        .context("No tag name provided.")?;

    let tag = context
        .assyst
        .database
        .get_tag(guild_id.try_into()?, name)
        .await?
        .context("No tag found.")?;

    context.reply_with_text(tag.data).await?;

    Ok(())
}

pub async fn run_tag_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let subcommand = args[0].as_text();

    match subcommand {
        "create" => run_create_subcommand(context, args).await,
        "delete" => run_delete_subcommand(context, args).await,
        "edit" => run_edit_subcommand(context, args).await,
        "list" => run_list_subcommand(context, args).await,
        "info" => run_info_subcommand(context, args).await,
        _ => run_tag_subcommand(context, args).await,
    }
}
