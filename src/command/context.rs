use bytes::Bytes;
use std::{error::Error, time::Instant};
use tokio::sync::Mutex;
use twilight_http::Client as HttpClient;
use twilight_model::{channel::Message, id::MessageId};

use crate::{caching::Reply, consts, Assyst};
use std::sync::Arc;

use super::messagebuilder::MessageBuilder;

pub struct Metrics {
    pub processing_time_start: Instant,
}

pub struct Context {
    pub assyst: Arc<Assyst>,
    pub message: Arc<Message>,
    pub metrics: Metrics,
    pub reply: Arc<Mutex<Reply>>,
}
impl Context {
    pub fn new(
        assyst: Arc<Assyst>,
        message: Arc<Message>,
        metrics: Metrics,
        reply: Arc<Mutex<Reply>>,
    ) -> Self {
        Context {
            assyst,
            message,
            metrics,
            reply,
        }
    }

    pub fn http(&self) -> &HttpClient {
        &self.assyst.http
    }

    pub async fn reply(
        &self,
        message_builder: MessageBuilder,
    ) -> Result<Arc<Message>, Box<dyn Error>> {
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
    ) -> Result<Arc<Message>, String> {
        let builder = MessageBuilder::new();
        if buffer.len() > consts::WORKING_FILESIZE_LIMIT_BYTES {
            let url = crate::rest::upload_to_filer(
                &self.assyst.reqwest_client,
                buffer,
                &format!("image/{}", format),
            )
            .await
            .map_err(|e| e.to_string())?;
            let builder = builder.content(&url);
            self.reply(builder).await.map_err(|e| e.to_string())
        } else {
            let builder = builder.attachment(&format!("attachment.{}", format), buffer.to_vec());
            self.reply(builder).await.map_err(|e| e.to_string())
        }
    }

    pub async fn reply_with_text(&self, text: &str) -> Result<Arc<Message>, String> {
        let builder = MessageBuilder::new().content(text);
        self.reply(builder).await.map_err(|e| e.to_string())
    }

    async fn create_new_message(
        &self,
        message_builder: MessageBuilder,
    ) -> Result<Arc<Message>, Box<dyn Error>> {
        let mut create_message = self
            .assyst
            .http
            .create_message(self.message.channel_id)
            .allowed_mentions()
            .build();
        if let Some(attachment) = message_builder.attachment {
            create_message = create_message.attachment(attachment.name, attachment.data.to_vec());
        };
        if let Some(content) = message_builder.content {
            create_message =
                create_message.content(&content[0..std::cmp::min(content.len(), 1999)])?;
        };
        if let Some(embed) = message_builder.embed {
            create_message = create_message.embed(embed)?;
        };
        let result = Arc::new(create_message.await?);
        Ok(result)
    }

    async fn edit_message(
        &self,
        message_id: MessageId,
        message_builder: MessageBuilder,
    ) -> Result<Arc<Message>, Box<dyn Error>> {
        let mut update_message = self
            .assyst
            .http
            .update_message(self.message.channel_id, message_id);

        match message_builder.content {
            Some(content) => {
                update_message = update_message.content(Some(
                    content[0..std::cmp::min(content.len(), 1999)].to_owned(),
                ))?
            }
            None => update_message = update_message.content(None)?,
        };

        match message_builder.embed {
            Some(embed) => update_message = update_message.embed(embed)?,
            None => update_message = update_message.embed(None)?,
        };

        let result = Arc::new(update_message.await?);
        Ok(result)
    }

    pub async fn reply_err(&self, content: &str) -> Result<Arc<Message>, Box<dyn Error>> {
        self.reply(
            MessageBuilder::new()
                .content(&format!(":warning: `{}`", content.replace("`", "'")))
                .clone(),
        )
        .await
    }
}
