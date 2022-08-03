use std::error::Error;

use assyst_common::{config::Config, consts::MESSAGE_CHARACTER_LIMIT, ansi::Ansi};
use twilight_http::Client as HttpClient;
use twilight_model::id::{marker::WebhookMarker, Id};
use twilight_util::builder::embed::EmbedBuilder;
use assyst_database::Database;

const CATEGORY_LOGS: i32 = 0;
const CATEGORY_COMMAND_USE: i32 = 1;

pub async fn panic(config: &Config, client: &HttpClient, message: &str) {
    let url = &config.logs.panic;

    if url.is_empty() {
        println!("PANIC: {}", message);
        return;
    }

    let content = format!("<@&{}>", config.logs.panic_notify_role);

    let _ = exec_webhook_with(client, Some(&content), url, message, 0xFF0000).await;
}

pub async fn fatal(config: &Config, database: &Database, message: &str) {
    if !config.db_logs {
        println!("really bad error: {}", message);
        return;
    };

    let er = format!("{} {}", "Error:".fg_bright_red(), message.fg_bright_red());

    let _ = database.log(&er, CATEGORY_LOGS).await;
}

pub async fn info(config: &Config, database: &Database, message: &str) {
    if !config.db_logs {
        println!("info: {}", message);
        return;
    };

    let _ = database.log(&message, CATEGORY_LOGS).await;
}

pub async fn guild_add(config: &Config, database: &Database, message: &str) {
    if !config.db_logs {
        println!("guild add: {}", message);
        return;
    };

    let message = format!("{} {}", "Added to guild:".fg_green(), message.fg_green());

    let _ = database.log(&message, CATEGORY_LOGS).await;
}

pub async fn command_use(config: &Config, database: &Database, message: &str) {
    if !config.db_logs {
        return;
    };

    let message = format!("Command used: {}", message);

    let _ = database.log(&message, CATEGORY_COMMAND_USE).await;
}

pub async fn log_vote(config: &Config, client: &HttpClient, message: &str) {
    let url: &str = config.logs.vote.as_ref();
    if url.is_empty() {
        println!("vote: {}", message);
        return;
    };

    let message = format!("**User Voted**: {}", message);

    let _ = exec_webhook_with(client, None, url, &message, 0xFFFFFF).await;
}

async fn exec_webhook_with(
    client: &HttpClient,
    content: Option<&str>,
    url: &str,
    message: &str,
    color: u32,
) -> Result<(), Box<dyn Error>> {
    let parts = url.split("/").collect::<Vec<&str>>();

    let (token, id) = (
        parts.iter().last().unwrap(),
        parts.iter().nth(parts.len() - 2).unwrap(),
    );

    let embed = EmbedBuilder::new()
        .description(
            message
                .chars()
                .take(MESSAGE_CHARACTER_LIMIT)
                .collect::<String>(),
        )
        .color(color)
        .build();

    let embeds = vec![embed];

    let mut builder = client
        .execute_webhook(Id::<WebhookMarker>::new(id.parse::<u64>().unwrap()), *token)
        .embeds(&embeds)
        .unwrap();

    if let Some(content) = content {
        builder = builder.content(content).unwrap();
    }

    builder.exec().await?;
    Ok(())
}
