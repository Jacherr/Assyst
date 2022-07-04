use assyst_common::persistent_cache::{CacheRequestData, CacheResponseData, CacheResponseInner};

use crate::{
    guild_cache::{handle_guild_create_event, handle_guild_delete_event, handle_ready_event},
    state::SharedState,
};

pub fn handle_request(state: SharedState, request: CacheRequestData) -> CacheResponseInner {
    match request {
        CacheRequestData::SendReadyEvent(event) => handle_ready_event(state, event),
        CacheRequestData::SendGuildCreate(event) => handle_guild_create_event(state, event),
        CacheRequestData::SendGuildDelete(event) => handle_guild_delete_event(state, event),
        CacheRequestData::GetTotalGuilds => Ok(CacheResponseData::TotalGuilds(
            state.borrow().guild_cache.guild_ids.len(),
        )),
        CacheRequestData::GetTopGuilds => Ok(CacheResponseData::TopGuilds(
            state.borrow().guild_cache.top_guilds.clone(),
        )),
    }
}
