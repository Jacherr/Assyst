use anyhow::Context as _;
use assyst_common::consts::{self, ABSOLUTE_INPUT_FILE_SIZE_LIMIT_BYTES};
use bytes::Bytes;
use std::time::Instant;
use tokio::sync::Mutex;
use twilight_http::Client as HttpClient;
use twilight_model::{
    channel::{embed::Embed, message::AllowedMentions, Message},
    http::attachment::Attachment as TwilightAttachment,
};

use crate::{
    caching::Reply,
    util::{MessageId, UserId},
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

    pub async fn reply(&self, message_builder: MessageBuilder) -> anyhow::Result<Arc<Message>> {
        let mut reply_lock = self.reply.lock().await;

        if reply_lock.invocation_deleted {
            return Err(anyhow::anyhow!(
                "The invocation for this command was deleted, so the result was discarded."
            ));
        }

        if !reply_lock.has_replied() {
            let result = self.create_new_message(message_builder).await?;
            reply_lock.set_reply(result.clone());
            Ok(result)
        } else {
            let reply = reply_lock.reply.as_ref().expect("No reply found");

            if reply.attachments.len() > 0 || message_builder.attachment.is_some() {
                self.http()
                    .delete_message(reply.channel_id, reply.id)
                    .exec()
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
    ) -> anyhow::Result<Arc<Message>> {
        self.reply_with_file(&format!("image/{}", format), buffer)
            .await
    }

    pub async fn reply_with_file(&self, mime: &str, buffer: Bytes) -> anyhow::Result<Arc<Message>> {
        self.reply_with_image_and_text(mime, buffer, None).await
    }

    pub async fn reply_with_image_and_text(
        &self,
        format: &str,
        buffer: Bytes,
        text: Option<String>,
    ) -> anyhow::Result<Arc<Message>> {
        let mut builder = MessageBuilder::new();
        let real_format = format.split("/").nth(1).context("Invalid format")?;

        if let Some(text) = text {
            let text = if text.is_empty() {
                String::from("[Empty Response]")
            } else {
                text
            };

            builder = builder.content(text.into_boxed_str());
        }

        if buffer.len() > consts::WORKING_FILESIZE_LIMIT_BYTES {
            if buffer.len() > ABSOLUTE_INPUT_FILE_SIZE_LIMIT_BYTES {
                return Err(anyhow::anyhow!(
                    "The output file exceeded the maximum file size limit of {}MB. Try using a smaller input.",
                    ABSOLUTE_INPUT_FILE_SIZE_LIMIT_BYTES / 1000 / 1000
                ));
            }
            let url = crate::rest::upload_to_filer(self.assyst.clone(), buffer, &format).await?;
            let builder = builder.content(url.into_boxed_str());
            self.reply(builder).await
        } else {
            let builder = builder.attachment(
                format!("attachment.{}", real_format).into_boxed_str(),
                buffer.to_vec(),
            );
            self.reply(builder).await
        }
    }

    pub async fn reply_with_text<S: Into<String>>(&self, text: S) -> anyhow::Result<Arc<Message>> {
        let text: String = text.into();
        // Trim the response so that a message with just spaces is considered empty
        // We only care about responses with just spaces, so trim_start and trim in this case do the same thing
        let trimmed = text.trim_start();

        let checked_text = if trimmed.is_empty() {
            String::from("[Empty Response]")
        } else {
            text
        };

        let builder = MessageBuilder::new().content(checked_text.into_boxed_str());
        self.reply(builder).await
    }

    async fn create_new_message(
        &self,
        message_builder: MessageBuilder,
    ) -> anyhow::Result<Arc<Message>> {
        let allowed_mentions = AllowedMentions::default();

        let mut create_message = self
            .assyst
            .http
            .create_message(self.message.channel_id)
            .allowed_mentions(Some(&allowed_mentions));

        let attachments: [TwilightAttachment; 1];

        if let Some(attachment) = message_builder.attachment {
            attachments = [TwilightAttachment::from_bytes(
                attachment.name.to_string(),
                attachment.data,
                0,
            )];
            create_message = create_message.attachments(&attachments)?;
        };
        let chars: String;
        if let Some(content) = message_builder.content {
            chars = content
                .chars()
                .take(consts::MESSAGE_CHARACTER_LIMIT)
                .collect::<String>();
            create_message = create_message.content(&chars)?
        };
        let embeds: [Embed; 1];
        if let Some(embed) = message_builder.embed {
            embeds = [embed];
            create_message = create_message.embeds(&embeds)?;
        };
        let message = create_message.exec().await?.model().await?;
        let result = Arc::new(message);
        Ok(result)
    }

    async fn edit_message(
        &self,
        message_id: MessageId,
        message_builder: MessageBuilder,
    ) -> anyhow::Result<Arc<Message>> {
        let mut update_message = self
            .assyst
            .http
            .update_message(self.message.channel_id, message_id);

        let chars: String;

        match message_builder.content {
            Some(content) => {
                chars = content.chars().take(1999).collect::<String>();
                update_message = update_message.content(Some(&chars))?
            }
            None => update_message = update_message.content(None)?,
        };

        let e: [Embed; 1];

        match message_builder.embed {
            Some(embed) => {
                e = [embed];
                update_message = update_message.embeds(Some(&e))?
            }
            None => update_message = update_message.embeds(Some(&[]))?,
        };

        let result = Arc::new(update_message.exec().await?.model().await?);
        Ok(result)
    }

    pub async fn reply_err<S: Into<String>>(&self, content: S) -> anyhow::Result<Arc<Message>> {
        let content = format!(":warning: `{}`", content.into().replace("`", "'"));

        self.reply(MessageBuilder::new().content(content.into_boxed_str()))
            .await
    }

    pub fn author_id(&self) -> UserId {
        self.message.author.id
    }
}
