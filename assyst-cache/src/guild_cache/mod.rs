use std::collections::HashSet;

use assyst_common::persistent_cache::{
    event_containers::{GuildCreateSend, GuildDeleteSend, ReadySend},
    guild_cache::TopGuilds,
    CacheResponseData, CacheResponseInner,
};

use crate::state::SharedState;

pub struct GuildCache {
    pub top_guilds: TopGuilds,
    pub guild_ids: HashSet<u64>,
}
impl GuildCache {
    pub fn new() -> GuildCache {
        GuildCache {
            top_guilds: TopGuilds::new(),
            guild_ids: HashSet::new(),
        }
    }
}

pub fn handle_ready_event(state: SharedState, event: ReadySend) -> CacheResponseInner {
    let mut new_guilds = 0;
    for guild in &event.guilds {
        if state.borrow_mut().guild_cache.guild_ids.insert(guild.id) {
            // count the number of new unique guilds,
            // so we can add a number of guilds to the metrics in one call
            // (one atomic store instead of a lot of them)
            new_guilds += 1;
        }
    }

    Ok(CacheResponseData::TotalNewGuilds(new_guilds as usize))
}

pub fn handle_guild_create_event(state: SharedState, event: GuildCreateSend) -> CacheResponseInner {
    let cache = &mut state.borrow_mut().guild_cache;

    cache.top_guilds.add_guild(
        event.id,
        event.name.clone(),
        event.member_count.unwrap_or(0) as u32,
    );

    if !cache.guild_ids.contains(&event.id) {
        cache.guild_ids.insert(event.id);
        Ok(CacheResponseData::ShouldLogGuildCreate(true))
    } else {
        Ok(CacheResponseData::ShouldLogGuildCreate(false))
    }
}

pub fn handle_guild_delete_event(state: SharedState, event: GuildDeleteSend) -> CacheResponseInner {
    let cache = &mut state.borrow_mut().guild_cache;

    if !event.unavailable {
        if cache.guild_ids.remove(&event.id) {
            Ok(CacheResponseData::ShouldLogGuildDelete(true))
        } else {
            Ok(CacheResponseData::ShouldLogGuildDelete(false))
        }
    } else {
        Ok(CacheResponseData::ShouldLogGuildDelete(false))
    }
}
