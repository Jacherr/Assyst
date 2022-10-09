use std::{convert::TryInto, sync::Arc, time::Duration};

use anyhow::{anyhow, ensure, Context as _};
use assyst_common::{consts, eval::FakeEvalImageResponse, util::UserId};
use assyst_tag as tag;
use lazy_static::lazy_static;
use std::fmt::Write;
use tag::ParseResult;

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
-t info <name>               :: Get information about a tag

Tag documentation: https://jacher.io/tags
"#;

lazy_static! {
    pub static ref TAG_COMMAND: Command = CommandBuilder::new("tag")
        .category(CATEGORY_NAME)
        .alias("t")
        .description(DESCRIPTION)
        .cooldown(Duration::from_secs(1))
        .availability(CommandAvailability::Public)
        .arg(Argument::String)
        .arg(Argument::Optional(Box::new(Argument::String)))
        .arg(Argument::Optional(Box::new(Argument::StringRemaining)))
        .usage("[create|delete|edit|list|info|raw|<tag name>] [<tag name>] [<tag content>]")
        .example("create test hello, this is a tag")
        .example("delete test")
        .example("edit test new content")
        .example("list")
        .example("list 2")
        .example("info test")
        .example("test")
        .example("raw test")
        .build();
}

async fn run_create_subcommand(context: Arc<Context>, args: Vec<ParsedArgument>) -> CommandResult {
    let author = context.message.author.id.get();
    let guild_id = context.message.guild_id.unwrap().get();
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
    let author = context.message.author.id.get();
    let guild_id = context.message.guild_id.unwrap().get();
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
    let author = context.message.author.id.get();
    let guild_id = context.message.guild_id.unwrap().get();
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
    let guild_id = context.message.guild_id.unwrap().get();
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
    let guild_id = context.message.guild_id.unwrap().get();
    let name = args
        .get(1)
        .map(|t| t.maybe_text())
        .flatten()
        .ok_or(anyhow!("No tag name provided."))?;

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

async fn run_raw_subcommand(context: Arc<Context>, args: Vec<ParsedArgument>) -> CommandResult {
    let guild_id = context.message.guild_id.unwrap().get();
    let name = args
        .get(1)
        .map(|t| t.maybe_text())
        .flatten()
        .ok_or(anyhow!("No tag name provided."))?;

    let tag = context
        .assyst
        .database
        .get_tag(guild_id.try_into()?, name)
        .await?
        .context("No tag found.")?;

    let raw = util::codeblock(&tag.data, "");

    context.reply_with_text(raw).await?;
    Ok(())
}

async fn run_tag_subcommand(context: Arc<Context>, args: Vec<ParsedArgument>) -> CommandResult {
    let guild_id = context.message.guild_id.unwrap().get();
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

    match output {
        Ok(ParseResult { attachment, output }) => {
            if let Some((buffer, ty)) = attachment {
                let output = (!output.is_empty()).then(|| output);

                context
                    .reply_with_image_and_text(ty.as_mime(), buffer, output)
                    .await?;
            } else {
                context.reply_with_text(output).await?;
            }
        }
        Err(e) => {
            let message = util::codeblock(&format!("{e:?}"), "");
            context.reply_with_text(message).await?;
        }
    };

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
        "raw" => run_raw_subcommand(context, args).await,
        _ => run_tag_subcommand(context, args).await,
    }
}

struct TagContext {
    tokio: tokio::runtime::Handle,
    ccx: Arc<Context>,
}

impl tag::Context for TagContext {
    fn execute_javascript(&self, code: &str) -> anyhow::Result<FakeEvalImageResponse> {
        let response = self
            .tokio
            .block_on(fake_eval(&self.ccx.assyst, code, true))?;

        Ok(response)
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
        let user_id = user_id.unwrap_or(self.ccx.message.author.id.get());

        let user = self
            .tokio
            .block_on(self.ccx.http().user(UserId::new(user_id)).exec())?;

        if user.status().get() == 404 {
            return Err(anyhow!("User not found"));
        }

        let user = self.tokio.block_on(user.model())?;

        Ok(util::get_avatar_url(&user))
    }

    fn download(&self, url: &str) -> anyhow::Result<String> {
        let assyst = &self.ccx.assyst;

        let content =
            downloader::download_content(assyst, url, consts::ABSOLUTE_INPUT_FILE_SIZE_LIMIT_BYTES);

        let content = self.tokio.block_on(content)?;

        Ok(String::from_utf8_lossy(&content).into_owned())
    }

    fn channel_id(&self) -> anyhow::Result<u64> {
        Ok(self.ccx.message.channel_id.get())
    }

    fn guild_id(&self) -> anyhow::Result<u64> {
        self.ccx
            .message
            .guild_id
            .context("Missing Guild ID")
            .map(|s| s.get())
    }

    fn user_id(&self) -> anyhow::Result<u64> {
        Ok(self.ccx.message.author.id.get())
    }

    fn user_tag(&self, id: Option<u64>) -> anyhow::Result<String> {
        if let Some(id) = id {
            let user = self
                .tokio
                .block_on(self.ccx.http().user(UserId::new(id)).exec())?;

            if user.status().get() == 404 {
                return Err(anyhow!("User not found"));
            }

            let user = self.tokio.block_on(user.model())?;

            Ok(util::format_tag(&user))
        } else {
            Ok(util::format_tag(&self.ccx.message.author))
        }
    }
}
