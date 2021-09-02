use crate::{
    command::{
        command::{Argument, Command, CommandBuilder, CommandError, ParsedArgument, ParsedFlags},
        context::Context,
        registry::CommandResult,
    },
    consts::{self, DEFAULT_COLORS},
    rest::{self, bt::bad_translate, bt::translate_single},
    util::{codeblock, ensure_guild_manager, normalize_emojis},
};
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
    pub static ref MONEY_COMMAND: Command = CommandBuilder::new("money")
        .public()
        .description("money")
        .example("money")
        .usage("money")
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
        .public()
        .description("search rule34.xxx")
        .example("anime")
        .usage("[query]")
        .cooldown(Duration::from_secs(4))
        .category(CATEGORY_NAME)
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
}

pub async fn run_bt_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let text = args[0].as_text();
    let text = normalize_emojis(text);
    let translated = bad_translate(&context.assyst.reqwest_client, &text)
        .await
        .map_err(|e| e.to_string())?;

    let output = format!("**Output**\n{}", translated.result.text);
    context.reply_with_text(&output).await?;
    Ok(())
}

pub async fn run_btdebug_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let text = args[0].as_text();
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

pub async fn run_money_command(
    context: Arc<Context>,
    _: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    context
        .reply_with_text("https://media.discordapp.net/stickers/874300577180418068.png")
        .await?;
    Ok(())
}

pub async fn run_ocrbt_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let image = args[0].as_text();
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

pub async fn run_ocrtr_command(
    context: Arc<Context>,
    args: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    let lang = args[0].as_text();
    let image = args[1].as_text();

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

pub async fn run_rule34_command(
    context: Arc<Context>,
    _: Vec<ParsedArgument>,
    _flags: ParsedFlags,
) -> CommandResult {
    tokio::time::sleep(Duration::from_millis(1500)).await;

    context
        .reply_err("450 Blocked By Windows Parental Controls")
        .await
        .map_err(|e| e.to_string())?;
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
        .ok_or_else(|| CommandError::new_boxed("This command can only be used in servers"))?;

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
                    .ok_or_else(|| CommandError::new_boxed("No color code provided"))??;

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

                for (name, color) in DEFAULT_COLORS.iter() {
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
                    let is_color_role = DEFAULT_COLORS.iter().any(|(name, _)| role.name.eq(name));

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
                    .reply_with_text(&format!(
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
                .ok_or_else(|| CommandError::new_boxed("No color name provided."))?;

            let role = context
                .assyst
                .database
                .remove_color_role(guild_id as i64, name)
                .await?
                .ok_or_else(|| CommandError::new_boxed("Color role does not exist"))?;

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
                .ok_or_else(|| CommandError::new_boxed("Color role does not exist"))?;

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
                .reply_with_text(&format!("Gave you the color role {}", name))
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

            context.reply_with_text(&content).await?;
        }
    };

    Ok(())
}
