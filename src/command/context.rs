use twilight_model::channel::Message;
use std::{error::Error, time::Instant};

use crate::Assyst;
use std::sync::Arc;

use super::messagebuilder::MessageBuilder;

pub struct Metrics {
    pub processing_time_start: Instant
}

pub struct Context {
    pub assyst: Arc<Assyst>,
    pub message: Arc<Message>,
    pub metrics: Metrics
}
impl Context {
    pub fn new(assyst: Arc<Assyst>, message: Arc<Message>, metrics: Metrics) -> Self {
        Context {
            assyst,
            message,
            metrics
        }
    }

    pub async fn reply(&self, message_builder: MessageBuilder) -> Result<Message, Box<dyn Error>> {
        let mut create_message = self.assyst.http.create_message(self.message.channel_id);
        if let Some(attachment) = message_builder.attachment {
            create_message = create_message.attachment(attachment.name, attachment.data.to_vec());
        };
        if let Some(content) = message_builder.content {
            create_message = create_message.content(&content[0..std::cmp::min(content.len(), 1999)])?;
        };
        if let Some(embed) = message_builder.embed {
            create_message = create_message.embed(embed)?;
        };
        let result = create_message.await?;
        Ok(result)
    }

    pub async fn reply_err(&self, content: &str) -> Result<Message, Box<dyn Error>> {
        self.reply(MessageBuilder::new().content(&format!(":warning: `{}`", content.replace("`", "'"))).clone()).await
    }
}