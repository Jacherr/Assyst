use crate::{logger, Assyst};
use std::sync::Arc;
use twilight_model::channel::message::Message;
use twilight_model::util::Timestamp;
use twilight_model::{channel::message::MessageType, gateway::payload::incoming::MessageUpdate};

pub async fn handle(assyst: Arc<Assyst>, message: Box<MessageUpdate>) -> () {
    if !should_handle_message(&message).await {
        return;
    };

    let converted_message = convert_message_update_to_message(*message);

    match converted_message {
        Some(c) => {
            if let Err(e) = assyst.handle_command(c, true).await {
                logger::fatal(&assyst, &format!("Command execution failed: {:?}", e)).await;
            }
        }
        _ => {}
    }
}

async fn should_handle_message(message: &Box<MessageUpdate>) -> bool {
    match &message.author {
        Some(a) => !a.bot && message.guild_id.is_some(),
        None => false,
    }
}

fn convert_message_update_to_message(event: MessageUpdate) -> Option<Message> {
    let attachments = event.attachments.unwrap_or_default();
    let author = event.author?;
    let content = event.content.unwrap_or_default();
    let embeds = event.embeds.unwrap_or_default();
    let kind = event.kind.unwrap_or_else(|| MessageType::Regular);
    let mention_everyone = event.mention_everyone.unwrap_or_default();
    let mention_roles = event.mention_roles.unwrap_or_default();
    let pinned = event.pinned.unwrap_or_default();
    let timestamp = event
        .timestamp
        .unwrap_or(Timestamp::parse("1970-01-01T01:01:01+00:00").unwrap());
    Some(Message {
        application_id: None,
        interaction: None,
        activity: None,
        application: None,
        attachments,
        author,
        channel_id: event.channel_id,
        content,
        edited_timestamp: event.edited_timestamp,
        embeds,
        flags: None,
        guild_id: event.guild_id,
        id: event.id,
        kind,
        member: None,
        mention_channels: vec![],
        mention_everyone,
        mention_roles,
        mentions: vec![],
        pinned,
        reactions: vec![],
        reference: None,
        referenced_message: None,
        sticker_items: vec![],
        timestamp,
        tts: false,
        webhook_id: None,
        components: vec![],
        thread: None,
        role_subscription_data: None
    })
}
