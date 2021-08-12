use bytes::Bytes;
use std::{error::Error, time::Instant};
use tokio::sync::Mutex;
use twilight_http::Client as HttpClient;
use twilight_model::{
    channel::{message::AllowedMentions, Message},
    id::{MessageId, UserId},
};

use crate::{
    caching::Reply,
    consts::{self, MESSAGE_CHARACTER_LIMIT},
    Assyst,
};
use std::sync::Arc;

use super::messagebuilder::MessageBuilder;

#[derive(Clone)]
pub struct Metrics {
    pub processing_time_start: Instant,
}

#[derive(Clone)]
pub struct Context {
    pub assyst: Arc<Assyst>,
    pub message: Arc<Message>,
    pub metrics: Metrics,
    pub prefix: String,
    pub reply: Arc<Mutex<Reply>>,
}
impl Context {
    pub fn new(
        assyst: Arc<Assyst>,
        message: Arc<Message>,
        metrics: Metrics,
        prefix: String,
        reply: Arc<Mutex<Reply>>,
    ) -> Self {
        Context {
            assyst,
            message,
            metrics,
            prefix,
            reply,
        }
    }

    pub fn http(&self) -> &HttpClient {
        &self.assyst.http
    }

    pub async fn reply(
        &self,
        message_builder: MessageBuilder,
    ) -> Result<Arc<Message>, Box<dyn Error + Send + Sync>> {
        let mut reply_lock = self.reply.lock().await;
        if !reply_lock.has_replied() {
            let result = self.create_new_message(message_builder).await?;
            reply_lock.set_reply(result.clone());
            Ok(result)
        } else {
            let reply = reply_lock.reply.as_ref().unwrap();
            if reply.attachments.len() > 0 || message_builder.attachment.is_some() {
                self.http()
                    .delete_message(reply.channel_id, reply.id)
                    .await?;
                let result = self.create_new_message(message_builder).await?;
                reply_lock.set_reply(result.clone());
                Ok(result)
            } else {
                let result = self.edit_message(reply.id, message_builder).await?;
                reply_lock.set_reply(result.clone());
                Ok(result)
            }
        }
    }

    pub async fn reply_with_image(
        &self,
        format: &str,
        buffer: Bytes,
    ) -> Result<Arc<Message>, Box<dyn Error + Send + Sync>> {
        let builder = MessageBuilder::new();
        if buffer.len() > consts::WORKING_FILESIZE_LIMIT_BYTES {
            let url = crate::rest::upload_to_filer(
                &self.assyst.reqwest_client,
                buffer,
                &format!("image/{}", format),
            )
            .await?;
            let builder = builder.content(&url);
            self.reply(builder).await
        } else {
            let builder = builder.attachment(&format!("attachment.{}", format), buffer.to_vec());
            self.reply(builder).await
        }
    }

    pub async fn reply_with_text(&self, text: &str) -> Result<Arc<Message>, Box<dyn Error + Send + Sync>> {
        let checked_text = if text.len() == 0 {
            "[Empty Response]"
        } else {
            text
        };
        let builder = MessageBuilder::new().content(checked_text);
        self.reply(builder).await
    }

    async fn create_new_message(
        &self,
        message_builder: MessageBuilder,
    ) -> Result<Arc<Message>, Box<dyn Error + Send + Sync>> {
        let mut create_message = self
            .assyst
            .http
            .create_message(self.message.channel_id)
            .allowed_mentions(AllowedMentions::default());

        if let Some(attachment) = message_builder.attachment {
            create_message = create_message.files([(attachment.name, attachment.data.to_vec())]);
        };
        if let Some(content) = message_builder.content {
            create_message = create_message.content(
                &content
                    .chars()
                    .take(MESSAGE_CHARACTER_LIMIT)
                    .collect::<String>(),
            )?
        };
        if let Some(embed) = message_builder.embed {
            create_message = create_message.embeds([embed])?;
        };
        let result = Arc::new(create_message.await?);
        Ok(result)
    }

    async fn edit_message(
        &self,
        message_id: MessageId,
        message_builder: MessageBuilder,
    ) -> Result<Arc<Message>, Box<dyn Error + Send + Sync>> {
        let mut update_message = self
            .assyst
            .http
            .update_message(self.message.channel_id, message_id);

        match message_builder.content {
            Some(content) => {
                update_message =
                    update_message.content(Some(content.chars().take(1999).collect::<String>()))?
            }
            None => update_message = update_message.content(None)?,
        };

        match message_builder.embed {
            Some(embed) => update_message = update_message.embeds([embed])?,
            None => update_message = update_message.embeds([])?,
        };

        let result = Arc::new(update_message.await?);
        Ok(result)
    }

    pub async fn reply_err(&self, content: &str) -> Result<Arc<Message>, Box<dyn Error + Send + Sync>> {
        self.reply(
            MessageBuilder::new()
                .content(&format!(":warning: `{}`", content.replace("`", "'")))
                .clone(),
        )
        .await
    }

    pub fn author_id(&self) -> UserId {
        self.message.author.id
    }
}
