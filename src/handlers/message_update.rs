use twilight_model::{channel::message::MessageType, gateway::payload::MessageUpdate};
use twilight_model::channel::message::Message;
use std::sync::Arc;
use crate::Assyst;

pub async fn handle(
    assyst: Arc<Assyst>,
    message: Box<MessageUpdate>
) -> () {
    if !should_handle_message(&message).await { return };
    let converted_message = convert_message_update_to_message(message.as_ref().clone());
    match converted_message {
        Some(c) => {
            if let Err(e) = assyst.handle_command(c).await {
                println!("Command execution failed: {:?}", e);
            }
        },
        _ => {}
    }
}

async fn should_handle_message(
    message: &Box<MessageUpdate>
) -> bool {
    match &message.author {
        Some(a) => !a.bot && a.discriminator != "0000" && message.guild_id.is_some(),
        None => false
    }
}

fn convert_message_update_to_message(event: MessageUpdate) -> Option<Message> {
    let attachments = if let Some(a) = event.attachments { a } else { vec![] };
    let author = if let Some(a) = event.author { a } else { return None };
    let content = if let Some(c) = event.content { c } else { String::from("") };
    let embeds = if let Some(e) = event.embeds { e } else { vec![] };
    let kind = if let Some(k) = event.kind { k } else { MessageType::Regular };
    let mention_everyone = if let Some(m) = event.mention_everyone { m } else { false };
    let mention_roles = if let Some(m) = event.mention_roles { m } else { vec![] };
    let pinned = if let Some(p) = event.pinned { p } else { false };
    let timestamp = if let Some(t) = event.timestamp { t } else { String::from("") };
    Some(Message {
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
        stickers: vec![],
        timestamp,
        tts: false,
        webhook_id: None
    })
}