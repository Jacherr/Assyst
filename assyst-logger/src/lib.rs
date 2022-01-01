use std::error::Error;

use assyst_common::{config::Config, consts::MESSAGE_CHARACTER_LIMIT};
use twilight_embed_builder::EmbedBuilder;
use twilight_http::Client as HttpClient;
use twilight_model::id::WebhookId;

pub async fn panic(config: &Config, client: &HttpClient, message: &str) {
    let url: &str = config.logs.fatal.as_ref();
    if url.is_empty() {
        println!("really bad error: {}", message);
        return;
    }

    let content = format!("<@&{}>", config.logs.panic_notify_role);

    let _ = exec_webhook_with(client, Some(&content), url, message, 0xFF0000).await;
}

pub async fn fatal(config: &Config, client: &HttpClient, message: &str) {
    let url: &str = config.logs.fatal.as_ref();
    if url.is_empty() {
        println!("really bad error: {}", message);
        return;
    };

    let er = format!("**really bad error**: {}", message);

    let _ = exec_webhook_with(client, None, url, &er, 0xFF0000).await;
}

pub async fn info(config: &Config, client: &HttpClient, message: &str) {
    let url: &str = config.logs.info.as_ref();
    if url.is_empty() {
        println!("info: {}", message);
        return;
    };

    let message = format!("**info**: {}", message);

    let _ = exec_webhook_with(client, None, url, &message, 0x00D0FF).await;
}

pub async fn guild_add(config: &Config, client: &HttpClient, message: &str) {
    let url: &str = config.logs.info.as_ref();
    if url.is_empty() {
        println!("guild add: {}", message);
        return;
    };

    let message = format!("**guild add**: {}", message);

    let _ = exec_webhook_with(client, None, url, &message, 0x3c200).await;
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
        .build()?;

    let mut builder = client
        .execute_webhook(WebhookId::from(id.parse::<u64>().unwrap()), *token)
        .embeds(vec![embed]);

    if let Some(content) = content {
        builder = builder.content(content);
    }

    builder.await?;
    Ok(())
}
