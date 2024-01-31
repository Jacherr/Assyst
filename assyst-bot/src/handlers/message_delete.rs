use crate::Assyst;
use serenity::all::MessageDeleteEvent;
use std::sync::Arc;
use twilight_model::{
    gateway::payload::incoming::MessageDelete,
    id::{
        marker::{ChannelMarker, MessageMarker},
        Id,
    },
};

pub async fn handle(assyst: Arc<Assyst>, message: MessageDeleteEvent) -> () {
    let try_reply = assyst
        .replies
        .read()
        .await
        .get_reply_from_invocation_id(Id::<MessageMarker>::new(message.message_id.get()))
        .await;
    if let Some(reply) = try_reply {
        let mut lock = reply.lock().await;
        lock.set_invocation_deleted();
        if !lock.in_use {
            if let Some(r) = &lock.reply {
                let _ = assyst
                    .http
                    .delete_message(Id::<ChannelMarker>::new(message.channel_id.get()), r.id)
                    .await;
            }
        }
    }
}
