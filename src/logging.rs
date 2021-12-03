use crate::consts::MESSAGE_CHARACTER_LIMIT;
use crate::Assyst;
use std::error::Error;
use twilight_embed_builder::EmbedBuilder;
use twilight_model::id::WebhookId;
pub struct Logger {}
impl Logger {
    pub async fn panic(&self, assyst: &Assyst, message: &str) {
        let url: &str = assyst.config.logs.fatal.as_ref();
        if url.is_empty() {
            println!("really bad error: {}", message);
            return;
        }

        let content = format!("<@&{}>", assyst.config.logs.panic_notify_role);

        let _ = self
            .exec_webhook_with(assyst, Some(&content), url, message, 0xFF0000)
            .await;
    }

    pub async fn fatal(&self, assyst: &Assyst, message: &str) {
        let url: &str = assyst.config.logs.fatal.as_ref();
        if url.is_empty() {
            println!("really bad error: {}", message);
            return;
        };

        let er = format!("**really bad error**: {}", message);

        let _ = self
            .exec_webhook_with(assyst, None, url, &er, 0xFF0000)
            .await;
    }

    pub async fn info(&self, assyst: &Assyst, message: &str) {
        let url: &str = assyst.config.logs.info.as_ref();
        if url.is_empty() {
            println!("info: {}", message);
            return;
        };

        let message = format!("**info**: {}", message);

        let _ = self
            .exec_webhook_with(assyst, None, url, &message, 0x00D0FF)
            .await;
    }

    pub async fn guild_add(&self, assyst: &Assyst, message: &str) {
        let url: &str = assyst.config.logs.info.as_ref();
        if url.is_empty() {
            println!("guild add: {}", message);
            return;
        };

        let message = format!("**guild add**: {}", message);

        let _ = self
            .exec_webhook_with(assyst, None, url, &message, 0x3c200)
            .await;
    }

    pub async fn log_vote(&self, assyst: &Assyst, message: &str) {
        let url: &str = assyst.config.logs.vote.as_ref();
        if url.is_empty() {
            println!("vote: {}", message);
            return;
        };

        let message = format!("**User Voted**: {}", message);

        let _ = self
            .exec_webhook_with(assyst, None, url, &message, 0xFFFFFF)
            .await;
    }

    async fn exec_webhook_with(
        &self,
        assyst: &Assyst,
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

        let mut builder = assyst
            .http
            .execute_webhook(WebhookId::from(id.parse::<u64>().unwrap()), *token)
            .embeds(vec![embed]);

        if let Some(content) = content {
            builder = builder.content(content);
        }

        builder.await?;
        Ok(())
    }
}
