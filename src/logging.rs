use crate::consts::MESSAGE_CHARACTER_LIMIT;
use crate::Assyst;
use std::error::Error;
use std::sync::Arc;
use twilight_embed_builder::EmbedBuilder;
use twilight_model::id::WebhookId;
pub struct Logger {}
impl Logger {
    pub async fn fatal(&self, assyst: Arc<Assyst>, message: &str) {
        let url: &str = assyst.config.logs.fatal.as_ref();
        if url.is_empty() {
            println!("really bad error: {}", message);
            return;
        };

        let er = format!("**really bad error**: {}", message);

        let _ = self
            .exec_webhook_with(assyst.clone(), url, &er, 0xFF0000)
            .await;
    }

    pub async fn info(&self, assyst: Arc<Assyst>, message: &str) {
        let url: &str = assyst.config.logs.info.as_ref();
        if url.is_empty() {
            println!("info: {}", message);
            return;
        };

        let message = format!("**info**: {}", message);

        let _ = self
            .exec_webhook_with(assyst.clone(), url, &message, 0x00D0FF)
            .await;
    }

    async fn exec_webhook_with(
        &self,
        assyst: Arc<Assyst>,
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

        assyst
            .http
            .execute_webhook(WebhookId::from(id.parse::<u64>().unwrap()), *token)
            .embeds(vec![embed])
            .await?;

        Ok(())
    }
}
