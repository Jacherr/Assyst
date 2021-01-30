use twilight_model::channel::Message;
use std::error::Error;

use crate::Assyst;
use std::sync::Arc;

use super::messagebuilder::MessageBuilder;
pub struct Context {
    assyst: Arc<Assyst>,
    message: Message
}
impl Context {
    pub async fn reply(&self, message_builder: MessageBuilder) -> Result<Message, Box<dyn Error>> {
        let mut create_message = self.assyst.http.create_message(self.message.channel_id);
        if let Some(attachment) = message_builder.attachment {
            create_message = create_message.attachment(attachment.name, attachment.data);
        };
        if let Some(content) = message_builder.content {
            create_message = create_message.content(content)?;
        };
        if let Some(embed) = message_builder.embed {
            create_message = create_message.embed(embed)?;
        };
        let result = create_message.await?;
        Ok(result)
    }
}