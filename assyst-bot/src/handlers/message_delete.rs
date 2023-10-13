use crate::Assyst;
use std::sync::Arc;
use twilight_model::gateway::payload::incoming::MessageDelete;

pub async fn handle(assyst: Arc<Assyst>, message: MessageDelete) -> () {
    let try_reply = assyst
        .replies
        .read()
        .await
        .get_reply_from_invocation_id(message.id)
        .await;
    if let Some(reply) = try_reply {
        let mut lock = reply.lock().await;
        lock.set_invocation_deleted();
        if !lock.in_use {
            if let Some(r) = &lock.reply {
                let _ = assyst
                    .http
                    .delete_message(message.channel_id, r.id)
                    .await;
            }
        }
    }
}
