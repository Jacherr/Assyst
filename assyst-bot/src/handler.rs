use crate::{handlers::*, logger, Assyst};
use std::sync::Arc;
use twilight_model::gateway::event::{GatewayEvent, DispatchEvent};

pub async fn handle_event(assyst: Arc<Assyst>, event: GatewayEvent) {
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
                    assyst
                        .add_guild_to_top_guilds(
                            guild.id.get(),
                            guild.name.clone(),
                            guild.member_count.unwrap_or(0) as u32,
                        )
                        .await;
        
                    if !assyst.guild_in_list(guild.id.get()).await {
                        assyst.add_guild_to_list(guild.id.get()).await;
        
                        if !guild.unavailable {
                            assyst.metrics.add_guild();
                        }
        
                        logger::guild_add(
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
                DispatchEvent::GuildDelete(guild) => {
                    if !guild.unavailable {
                        let is_in = assyst.remove_guild_from_list(guild.id.get()).await;
        
                        if is_in {
                            assyst.metrics.delete_guild();
                            logger::info(&assyst, &format!("Removed from guild: {}", guild.id)).await;
                        }
                    }
                }
                DispatchEvent::Ready(r) => {
                    let mut new_guilds = 0;
                    for guild in &r.guilds {
                        if assyst.add_guild_to_list(guild.id.get()).await {
                            // count the number of new unique guilds,
                            // so we can add a number of guilds to the metrics in one call
                            // (one atomic store instead of a lot of them)
                            new_guilds += 1;
                        }
                    }
        
                    assyst.metrics.add_guilds(new_guilds);
        
                    logger::info(
                        &assyst,
                        &format!(
                            "Shard {}: READY in {} guilds",
                            r.shard.unwrap_or_default()[0],
                            r.guilds.len()
                        ),
                    )
                    .await;
                }
                _ => {}
            }
        }
        _ => {}
    }
}
