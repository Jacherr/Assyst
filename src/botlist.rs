use crate::assyst::Assyst;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::{sync::Arc};
use crate::consts::BOT_ID;

const VOTE_FREE_TIER_1_REQUESTS: usize = 5;

pub struct BotList {
    pub webhook_route: &'static str,
    pub post_guilds_url: String,
}

#[derive(Deserialize)]
pub struct DiscordBotListWebhookBody {
    admin: Option<bool>,
    avatar: Option<String>,
    username: String,
    id: String,
}

lazy_static! {
    pub static ref DISCORD_BOT_LIST: BotList = BotList {
        webhook_route: "dbl",
        post_guilds_url: format!("https://discordbotlist.com/api/v1/bots/{}/stats", BOT_ID),
    };
}

mod filters {
    use std::sync::Arc;
    use warp::{Filter, Rejection, Reply};
    use crate::assyst::Assyst;
    use super::{DISCORD_BOT_LIST, handlers};

    pub fn root() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::path::end().and(warp::get()).and_then(handlers::root)
    }

    pub fn dbl(assyst: Arc<Assyst>, auth: &'static str) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::path(DISCORD_BOT_LIST.webhook_route)
            .and(warp::post())
            .and(warp::header::exact("authorization", auth))
            .and(warp::body::json())
            .and(warp::any().map(move || assyst.clone()))
            .and_then(handlers::dbl)
    }
}

mod handlers {
    use super::DiscordBotListWebhookBody;
    use warp::{Rejection, Reply};
    use crate::assyst::Assyst;
    use std::sync::Arc;

    pub async fn root() -> Result<impl Reply, Rejection> {
        Ok(warp::reply::reply())
    }

    pub async fn dbl(body: DiscordBotListWebhookBody, assyst: Arc<Assyst>) -> Result<impl Reply, Rejection> {
        
        Ok(warp::reply::reply())
    }
}

pub struct BotListManager {
    assyst: Arc<Assyst>,
}
impl BotListManager {
    pub fn new(assyst: Arc<Assyst>) -> Self {
        Self { assyst }
    }

    pub fn init(&self) {
        todo!()
    }
}
