use crate::{
    caching::persistent_caching::{
        get_new_guilds_from_ready, handle_guild_create_event, handle_guild_delete_event,
    },
    handlers::*,
    logger, Assyst,
};
use anyhow::Context;
use serenity::all::{Event, ShardInfo, ShardId};
use std::sync::Arc;
use twilight_model::gateway::event::{DispatchEvent, GatewayEvent};

pub async fn handle_event(assyst: Arc<Assyst>, event: Event) -> anyhow::Result<()> {
    match event {
        Event::MessageCreate(message) => {
            message_create::handle(assyst, Box::new(message)).await;
        }
        Event::MessageDelete(message) => {
            message_delete::handle(assyst, message).await;
        }
        Event::MessageUpdate(message) => {
            message_update::handle(assyst, Box::new(message)).await;
        }
        Event::GuildCreate(guild) => { 
            let id = guild.guild.id;
            let name = guild.guild.name.clone();
            let member_count = guild.guild.member_count;
            let should_log = handle_guild_create_event(assyst.clone(), guild)
                .await
                .context("failed to handle guild create")?;

            if should_log {
                assyst.metrics.add_guild();

                logger::guild_add(
                    &assyst,
                    &format!("{} ({}) ({} members)", name, id.get(), member_count),
                )
                .await;
            }
        }
        Event::GuildDelete(guild) => {
            let id = guild.guild.id;
            let should_log = handle_guild_delete_event(assyst.clone(), guild)
                .await
                .context("failed to handle guild delete")?;
            if should_log {
                assyst.metrics.delete_guild();
                logger::guild_remove(&assyst, &format!("{}", id.get())).await;
            }
        }
        Event::Ready(r) => {
            let shard = r.ready.shard.unwrap_or(ShardInfo { id: ShardId(0), total: 1});
            let guilds = r.ready.guilds.len();
            let new_guilds = get_new_guilds_from_ready(assyst.clone(), r)
                .await
                .context("failed to handle guild ready")?;

            assyst.metrics.add_guilds(new_guilds as i64);

            logger::info(
                &assyst,
                &format!("Shard {}: READY in {} guilds", shard.id, guilds),
            )
            .await;
        }
        _ => {}
    }
    Ok(())
}