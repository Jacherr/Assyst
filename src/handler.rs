use crate::{Assyst, handlers::*};
use twilight_gateway::Event;
use std::sync::Arc;

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
        Event::ShardConnected(d) => {
            println!("Shard {}: READY", d.shard_id);
        }
        Event::ShardDisconnected(d) => {
            println!(
                "Shard {}: DISCONNECTED, {:?}",
                d.shard_id,
                d.reason.to_owned()
            );
        }
        _ => {}
    }
}
