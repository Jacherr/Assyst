use serde::{Deserialize, Serialize};
use twilight_model::{
    gateway::payload::incoming::{GuildCreate, GuildDelete, Ready},
    guild::{Guild, UnavailableGuild},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct ReadyGuild {
    pub id: u64,
}
impl From<Guild> for ReadyGuild {
    fn from(g: Guild) -> ReadyGuild {
        ReadyGuild { id: g.id.get() }
    }
}
impl From<UnavailableGuild> for ReadyGuild {
    fn from(g: UnavailableGuild) -> ReadyGuild {
        ReadyGuild { id: g.id.get() }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReadySend {
    pub guilds: Vec<ReadyGuild>,
}
impl From<Ready> for ReadySend {
    fn from(ready: Ready) -> Self {
        let g = ready
            .guilds
            .into_iter()
            .map(|x| ReadyGuild::from(x))
            .collect::<Vec<_>>();

        ReadySend { guilds: g }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildDeleteSend {
    pub id: u64,
    pub unavailable: bool,
}
impl From<GuildDelete> for GuildDeleteSend {
    fn from(guild_delete: GuildDelete) -> Self {
        GuildDeleteSend {
            id: guild_delete.id.get(),
            unavailable: guild_delete.unavailable,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildCreateSend {
    pub id: u64,
    pub name: String,
    pub member_count: Option<u64>,
}
impl From<GuildCreate> for GuildCreateSend {
    fn from(guild_create: GuildCreate) -> Self {
        GuildCreateSend {
            id: guild_create.id.get(),
            name: guild_create.name.clone(),
            member_count: guild_create.member_count,
        }
    }
}
