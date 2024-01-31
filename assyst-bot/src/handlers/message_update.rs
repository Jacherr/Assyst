use crate::{logger, Assyst};
use serenity::all::MessageUpdateEvent;
use std::sync::Arc;
use twilight_model::channel::message::Message;
use twilight_model::util::Timestamp;
use twilight_model::{channel::message::MessageType, gateway::payload::incoming::MessageUpdate};

use super::ser_message_update_to_twl_message;

pub async fn handle(assyst: Arc<Assyst>, message: Box<MessageUpdateEvent>) -> () {
    if !should_handle_message(&message).await {
        return;
    };

    let converted_message = ser_message_update_to_twl_message(*message);

    match converted_message {
        Some(c) => {
            if let Err(e) = assyst.handle_command(c, true).await {
                logger::fatal(&assyst, &format!("Command execution failed: {:?}", e)).await;
            }
        }
        _ => {}
    }
}

async fn should_handle_message(message: &Box<MessageUpdateEvent>) -> bool {
    match &message.author {
        Some(a) => !a.bot && message.guild_id.is_some(),
        None => false,
    }
}
