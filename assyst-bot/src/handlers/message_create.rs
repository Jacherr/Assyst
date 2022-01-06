use crate::Assyst;
use std::sync::Arc;
use twilight_model::gateway::payload::MessageCreate;

pub async fn handle(assyst: Arc<Assyst>, message: Box<MessageCreate>) {
    // Bad translate channel
    if assyst.badtranslator.is_channel(message.channel_id.0).await {
        assyst.badtranslator.handle_message(&assyst, message).await;
        return;
    }

    // Regular commands
    if !should_handle_message(&message).await {
        return;
    }

    if let Err(e) = assyst.handle_command(message.0, false).await {
        assyst
            .logger
            .fatal(&assyst, &format!("Command execution failed: {:?}", e))
            .await;
    }
}

async fn should_handle_message(message: &MessageCreate) -> bool {
    !message.author.bot && message.author.discriminator != "0000" && message.guild_id.is_some()
}
