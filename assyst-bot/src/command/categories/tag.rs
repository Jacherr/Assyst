use std::{convert::TryInto, sync::Arc, time::Duration};

use anyhow::{ensure, Context as _};
use assyst_common::consts;
use assyst_tag as tag;
use lazy_static::lazy_static;
use std::fmt::Write;

use crate::{
    command::{
        command::{
            Argument, Command, CommandAvailability, CommandBuilder, ParsedArgument, ParsedFlags,
        },
        context::Context,
        parse::image_lookups::previous_message_attachment,
        registry::CommandResult,
    },
    downloader,
    rest::fake_eval,
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
        .availability(CommandAvailability::Private)
        .cooldown(Duration::from_secs(1))
        .arg(Argument::String)
        .arg(Argument::Optional(Box::new(Argument::String)))
        .arg(Argument::Optional(Box::new(Argument::StringRemaining)))
        .usage("[create|delete|edit|list|info|<tag name>] [<tag name>] [<tag content>]")
        .example("create test hello, this is a tag")
        .example("delete test")
        .example("edit test new content")
        .example("list")
        .example("list 2")
        .example("info test")
        .example("test")
        .build();
}

async fn run_create_subcommand(context: Arc<Context>, args: Vec<ParsedArgument>) -> CommandResult {
    let author = context.message.author.id.0;
    let guild_id = context.message.guild_id.unwrap().0;
    let name = args
        .get(1)
        .and_then(|t| t.maybe_text())
        .context("No tag name provided.")?;

    ensure!(name.len() < 20, "Tag name must be less than 20 characters");

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

    ensure!(page >= 1, "Page must be greater or equal to 1");

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

    let ccx = context.clone();
    let output = tokio::task::spawn_blocking(move || {
        let args = args
            .iter()
            .skip(1)
            .flat_map(|a| a.maybe_text())
            .map(|s| s.split_ascii_whitespace())
            .flatten()
            .collect::<Vec<_>>();

        let tokio = tokio::runtime::Handle::current();

        tag::parse(&tag.data, &args, TagContext { ccx, tokio })
    })
    .await?
    .context("Tag execution failed");

    let output = output.unwrap_or_else(|e| util::codeblock(&format!("{:?}", e), ""));

    context.reply_with_text(output).await?;

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

struct TagContext {
    tokio: tokio::runtime::Handle,
    ccx: Arc<Context>,
}

impl tag::Context for TagContext {
    fn execute_javascript(&self, code: &str) -> anyhow::Result<String> {
        let response = self.tokio.block_on(fake_eval(&self.ccx.assyst, code))?;

        Ok(response.message)
    }

    fn get_last_attachment(&self) -> anyhow::Result<String> {
        let http = &self.ccx.assyst.http;
        let message = &*self.ccx.message;
        let previous = self
            .tokio
            .block_on(previous_message_attachment(http, message))
            .context("Failed to extract last attachment")?;

        Ok(previous.into_owned())
    }

    fn get_avatar(&self, user_id: Option<u64>) -> anyhow::Result<String> {
        let http = &self.ccx.assyst.http;
        let user_id = user_id.unwrap_or(self.ccx.message.author.id.0);

        let user = self
            .tokio
            .block_on(http.user(user_id.into()))?
            .context("User not found")?;

        Ok(util::get_avatar_url(&user))
    }

    fn download(&self, url: &str) -> anyhow::Result<String> {
        let assyst = &self.ccx.assyst;

        let content =
            downloader::download_content(assyst, url, consts::ABSOLUTE_INPUT_FILE_SIZE_LIMIT_BYTES);

        let content = self.tokio.block_on(content)?;

        Ok(String::from_utf8_lossy(&content).into_owned())
    }
}
