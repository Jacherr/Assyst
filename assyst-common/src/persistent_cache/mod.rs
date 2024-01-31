pub mod event_containers;
pub mod guild_cache;

use serde::{Deserialize, Serialize};

use self::{
    event_containers::{GuildCreateSend, GuildDeleteSend, ReadySend},
    guild_cache::TopGuilds,
};

#[derive(Serialize, Deserialize, Debug)]
pub enum CacheRequestData {
    GetTopGuilds,
    GetTotalGuilds,
    SendReadyEvent(ReadySend),
    SendGuildCreate(GuildCreateSend),
    SendGuildDelete(GuildDeleteSend),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CacheRequest {
    id: usize,
    data: CacheRequestData,
}
impl CacheRequest {
    pub fn new(id: usize, data: CacheRequestData) -> CacheRequest {
        CacheRequest { id, data }
    }
    pub fn id(&self) -> usize {
        self.id
    }
    pub fn data(self) -> CacheRequestData {
        self.data
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CacheResponseData {
    TopGuilds(TopGuilds),
    ShouldLogGuildCreate(bool),
    ShouldLogGuildDelete(bool),
    TotalNewGuilds(usize),
    TotalGuilds(usize),
    GenericAck,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CacheResponse {
    id: usize,
    data: CacheResponseInner,
}
impl CacheResponse {
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn data(self) -> CacheResponseInner {
        self.data
    }

    pub fn new(id: usize, data: CacheResponseInner) -> Self {
        CacheResponse { id, data }
    }

    pub fn new_err(id: usize, error: CacheError) -> Self {
        CacheResponse {
            id,
            data: Err(error),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CacheError {
    CacheServerDied,
    CacheServerDown,
    Timeout,
}

impl std::fmt::Display for CacheError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CacheError::CacheServerDied => write!(f, "The cache server died"),
            CacheError::CacheServerDown => write!(f, "The cache server is down"),
            CacheError::Timeout => write!(f, "A timeout occurred"),
        }
    }
}

impl std::error::Error for CacheError {}

pub type CacheResponseInner = Result<CacheResponseData, CacheError>;
