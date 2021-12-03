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
        Event::GuildCreate(guild) => {
            if !assyst.guild_in_list(guild.id.0).await {
                assyst
                    .logger
                    .info(
                        &assyst,
                        &format!(
                            "{} ({}) ({} members)",
                            guild.name,
                            guild.id,
                            guild.member_count.unwrap_or(0)
                        ),
                    )
                    .await;
            }
        }
        Event::GuildDelete(guild) => {
            if assyst.guild_in_list(guild.id.0).await && !guild.unavailable {
                assyst
                    .logger
                    .info(&assyst, &format!("Removed from guild: {}", guild.id))
                    .await;
            }
        }
        Event::Ready(r) => {
            for guild in &r.guilds {
                assyst.add_guild_to_list(guild.id.0).await;
            }
            assyst
                .logger
                .info(
                    &assyst,
                    &format!(
                        "Shard {}: READY in {} guilds",
                        r.shard.unwrap_or_default()[0],
                        r.guilds.len()
                    ),
                )
                .await;
        }
        Event::ShardConnected(d) => {
            /* 
            assyst
                .logger
                .info(&assyst, &format!("Shard {}: CONNECTED", d.shard_id))
                .await;*/
        }
        Event::ShardDisconnected(d) => {
            /* 
            assyst
                .logger
                .info(
                    &assyst,
                    &format!(
                        "Shard {}: DISCONNECTED, {:?}",
                        d.shard_id,
                        d.reason.to_owned()
                    ),
                )
                .await;*/
        }
        Event::ShardReconnecting(r) => {
            /*assyst
                .logger
                .info(&assyst, &format!("Shard {}: RECONNECTING", r.shard_id))
                .await;*/
        }
        _ => {}
    }
}
