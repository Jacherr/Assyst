use crate::{handlers::*, logger, Assyst, caching::persistent_caching::{handle_guild_delete_event, get_new_guilds_from_ready, handle_guild_create_event}};
use std::sync::Arc;
use anyhow::Context;
use twilight_model::gateway::event::{GatewayEvent, DispatchEvent};

pub async fn handle_event(assyst: Arc<Assyst>, event: GatewayEvent) -> anyhow::Result<()> {
    match event {
        GatewayEvent::Dispatch(_, e) => {
            match *e {
                DispatchEvent::MessageCreate(message) => {
                    message_create::handle(assyst, message).await;
                }
                DispatchEvent::MessageDelete(message) => {
                    message_delete::handle(assyst, message).await;
                }
                DispatchEvent::MessageUpdate(message) => {
                    message_update::handle(assyst, message).await;
                }
                DispatchEvent::GuildCreate(guild) => {
                    let id = guild.id;
                    let name = guild.name.clone();
                    let member_count = guild.member_count.unwrap_or(0);
                    let should_log = handle_guild_create_event(assyst.clone(), *guild).await.context("failed to handle guild create")?;
        
                    if should_log {
                        assyst.metrics.add_guild();
        
                        logger::guild_add(
                            &assyst,
                            &format!(
                                "{} ({}) ({} members)",
                                name,
                                id,
                                member_count
                            ),
                        )
                        .await;
                    }
                }
                DispatchEvent::GuildDelete(guild) => {
                    let id = guild.id;
                    let should_log = handle_guild_delete_event(assyst.clone(), guild).await.context("failed to handle guild delete")?;
                    if should_log {
                        assyst.metrics.delete_guild();
                        logger::guild_remove(&assyst, &format!("{}", id)).await;
                    }
                }
                DispatchEvent::Ready(r) => {
                    let shard = r.shard.unwrap_or_default()[0];
                    let guilds = r.guilds.len();
                    let new_guilds = get_new_guilds_from_ready(assyst.clone(), *r).await.context("failed to handle guild ready")?;
        
                    assyst.metrics.add_guilds(new_guilds as i64);
        
                    logger::info(
                        &assyst,
                        &format!(
                            "Shard {}: READY in {} guilds",
                            shard,
                            guilds
                        ),
                    )
                    .await;
                }
                _ => {}
            }
        }
        _ => {}
    };

    Ok(())
}
