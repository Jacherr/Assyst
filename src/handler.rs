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
            println!("Shards {:?}: READY", r.shard.unwrap_or_default())
        }
        Event::ShardConnected(d) => {
            println!("Shard {}: CONNECTED", d.shard_id);
        },
        Event::ShardDisconnected(d) => {
            println!(
                "Shard {}: DISCONNECTED, {:?}",
                d.shard_id,
                d.reason.to_owned()
            );
        }
        Event::ShardReconnecting(r) => {
            println!("Shard {}: RECONNECTING", r.shard_id);
        }
        _ => {}
    }
}
