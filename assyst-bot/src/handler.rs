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
            if !guild.unavailable {
                assyst.metrics.read().await.processing.add_guild();
            }

            if !assyst.guild_in_list(guild.id.0).await {
                assyst
                    .logger
                    .guild_add(
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
            if !guild.unavailable && guild.id.0 != 907706584791150622 /* always get this on startup, no idea */ {
                assyst.metrics.read().await.processing.delete_guild();

                assyst
                    .logger
                    .info(&assyst, &format!("Removed from guild: {}", guild.id))
                    .await;

                assyst.remove_guild_from_list(guild.id.0).await;
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
        Event::ShardConnected(_d) => {
            /*
            assyst
                .logger
                .info(&assyst, &format!("Shard {}: CONNECTED", d.shard_id))
                .await;*/
        }
        Event::ShardDisconnected(_d) => {
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
        Event::ShardReconnecting(_r) => {
            /*assyst
            .logger
            .info(&assyst, &format!("Shard {}: RECONNECTING", r.shard_id))
            .await;*/
        }
        _ => {}
    }
}