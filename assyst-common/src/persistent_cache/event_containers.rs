use serde::{Deserialize, Serialize};
use serenity::all::{Guild, GuildCreateEvent, GuildDeleteEvent, ReadyEvent, UnavailableGuild};
use twilight_model::gateway::payload::incoming::{GuildCreate, GuildDelete, Ready};

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
impl From<ReadyEvent> for ReadySend {
    fn from(ready: ReadyEvent) -> Self {
        let g = ready
            .ready
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
impl From<GuildDeleteEvent> for GuildDeleteSend {
    fn from(guild_delete: GuildDeleteEvent) -> Self {
        GuildDeleteSend {
            id: guild_delete.guild.id.get(),
            unavailable: guild_delete.guild.unavailable,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildCreateSend {
    pub id: u64,
    pub name: String,
    pub member_count: Option<u64>,
}
impl From<GuildCreateEvent> for GuildCreateSend {
    fn from(guild_create: GuildCreateEvent) -> Self {
        GuildCreateSend {
            id: guild_create.guild.id.get(),
            name: guild_create.guild.name.clone(),
            member_count: Some(guild_create.guild.member_count),
        }
    }
}
