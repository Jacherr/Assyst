use crate::Assyst;
use std::sync::Arc;
use twilight_model::gateway::payload::MessageDelete;

pub async fn handle(assyst: Arc<Assyst>, message: MessageDelete) -> () {
    let try_reply = assyst
        .replies
        .read()
        .await
        .get_reply_from_invocation_id(message.id)
        .await;
    if let Some(reply) = try_reply {
        let lock = reply.lock().await;
        if !lock.in_use {
            if let Some(r) = &lock.reply {
                let _ = assyst.http.delete_message(message.channel_id, r.id).await;
            }
        }
    }
}
