use crate::{handlers::*, Assyst};
use std::sync::Arc;
use twilight_gateway::Event;

pub async fn handle_event(assyst: Arc<Assyst>, event: Event) {
    match event {
        Event::MessageCreate(message) => {
            message_create::handle(assyst, message).await;
        }
        Event::MessageDelete(message) => {
            message_delete::handle(assyst, message).await;
        }
        Event::MessageUpdate(message) => {
            message_update::handle(assyst, message).await;
        }
        Event::Ready(r) => {
            assyst
                .logger
                .info(
                    assyst.clone(),
                    &format!(
                        "Shard {}: READY in {} guilds",
                        r.shard.unwrap_or_default()[0],
                        r.guilds.len()
                    ),
                )
                .await;
        }
        Event::ShardConnected(d) => {
            assyst
                .logger
                .info(assyst.clone(), &format!("Shard {}: CONNECTED", d.shard_id))
                .await;
        }
        Event::ShardDisconnected(d) => {
            assyst
                .logger
                .info(
                    assyst.clone(),
                    &format!(
                        "Shard {}: DISCONNECTED, {:?}",
                        d.shard_id,
                        d.reason.to_owned()
                    ),
                )
                .await;
        }
        Event::ShardReconnecting(r) => {
            assyst
                .logger
                .info(
                    assyst.clone(),
                    &format!("Shard {}: RECONNECTING", r.shard_id),
                )
                .await;
        }
        _ => {}
    }
}
