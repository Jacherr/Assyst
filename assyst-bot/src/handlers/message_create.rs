use crate::{logger, Assyst};
use std::sync::Arc;
use twilight_model::gateway::payload::incoming::MessageCreate;

pub async fn handle(assyst: Arc<Assyst>, message: Box<MessageCreate>) {
    // Bad translate channel
    if assyst
        .badtranslator
        .is_channel(message.channel_id.get())
        .await
    {
        let result = assyst.badtranslator.handle_message(&assyst, message).await;
        handle_result(&assyst, result, "BT execution failed").await;
        return;
    }

    // Regular commands
    if !should_handle_message(&message).await {
        return;
    }

    if message.author.id.to_string() == "97153209843335168" {
        println!("test");
    } else if message.author.id.to_string() == "233667448887312385" {
        println!("test2");
    }

    let result = assyst.handle_command(message.0, false).await;
    handle_result(&assyst, result, "Command execution failed").await;
}

async fn handle_result<T>(assyst: &Assyst, result: anyhow::Result<T>, message: &str) {
    if let Err(e) = result {
        logger::fatal(assyst, &format!("{}: {:?}", message, e)).await;
    }
}

async fn should_handle_message(message: &MessageCreate) -> bool {
    !message.author.bot && message.guild_id.is_some()
}
