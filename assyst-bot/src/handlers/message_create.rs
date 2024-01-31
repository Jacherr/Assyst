use crate::{logger, Assyst};
use serenity::all::MessageCreateEvent;
use std::sync::Arc;
use twilight_model::{channel::Message, gateway::payload::incoming::MessageCreate};

use super::ser_message_to_twl_message;

pub async fn handle(assyst: Arc<Assyst>, message: Box<MessageCreateEvent>) {
    let message = ser_message_to_twl_message(message.message);

    // Bad translate channel
    if assyst
        .badtranslator
        .is_channel(message.channel_id.get())
        .await
    {
        let result = assyst
            .badtranslator
            .handle_message(&assyst, Box::new(message))
            .await;
        handle_result(&assyst, result, "BT execution failed").await;
        return;
    }

    // Regular commands
    if !should_handle_message(&message).await {
        return;
    }

    let result = assyst.handle_command(message, false).await;
    handle_result(&assyst, result, "Command execution failed").await;
}

async fn handle_result<T>(assyst: &Assyst, result: anyhow::Result<T>, message: &str) {
    if let Err(e) = result {
        logger::fatal(assyst, &format!("{}: {:?}", message, e)).await;
    }
}

async fn should_handle_message(message: &Message) -> bool {
    !message.author.bot && message.guild_id.is_some()
}
