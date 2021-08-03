// TODO

use crate::assyst::Assyst;
use serde::Deserialize;
use std::sync::Arc;
use twilight_model::id::UserId;

const VOTE_FREE_TIER_1_REQUESTS: i64 = 5;

#[derive(Deserialize)]
pub struct DiscordBotListWebhookBody {
    admin: Option<bool>,
    avatar: Option<String>,
    username: String,
    id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopGgWebhookBody {
    bot: String,
    user: String,
    r#type: String,
    is_weekend: bool,
    query: Option<String>,
}

pub async fn handle_vote(assyst: Arc<Assyst>, user_id: i64, service: &'static str) {
    let result = assyst
        .database
        .add_free_tier_1_requests(user_id, VOTE_FREE_TIER_1_REQUESTS)
        .await;

    if let Err(e) = result {
        assyst
            .logger
            .fatal(
                assyst.clone(),
                &format!(
                    "failed to give free tier 1 requests to voter, reason: {}",
                    e.to_string()
                ),
            )
            .await;
    } else {
        let message = assyst
            .http
            .user(UserId::from(user_id as u64))
            .await
            .map(|u| match u {
                Some(u) => format!(
                    "{}#{} voted for Assyst on {} and got {} free tier 1 requests!",
                    u.name, u.discriminator, service, VOTE_FREE_TIER_1_REQUESTS
                ),
                None => format!(
                    "An unknown user voted for Assyst on {} and got {} free tier 1 requests!",
                    service, VOTE_FREE_TIER_1_REQUESTS
                ),
            })
            .unwrap();

        assyst.logger.log_vote(assyst.clone(), &message).await;
    }
}

mod filters {
    use super::handlers;
    use crate::{assyst::Assyst, util::to_static_str};
    use std::sync::Arc;
    use warp::{Filter, Rejection, Reply};

    const DISCORD_BOT_LIST_ENDPOINT: &str = "dbl";
    const TOP_GG_ENDPOINT: &str = "topgg";

    pub fn root() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::path::end().and(warp::get()).and_then(handlers::root)
    }

    pub fn dbl(
        assyst: Arc<Assyst>,
    ) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::path(DISCORD_BOT_LIST_ENDPOINT)
            .and(warp::post())
            .and(warp::header::exact(
                "authorization",
                to_static_str(&assyst.config.auth.bot_list_webhook),
            ))
            .and(warp::body::json())
            .and(warp::any().map(move || assyst.clone()))
            .and_then(handlers::dbl)
    }

    pub fn topgg(
        assyst: Arc<Assyst>,
    ) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::path(TOP_GG_ENDPOINT)
            .and(warp::post())
            .and(warp::header::exact(
                "authorization",
                to_static_str(&assyst.config.auth.bot_list_webhook),
            ))
            .and(warp::body::json())
            .and(warp::any().map(move || assyst.clone()))
            .and_then(handlers::topgg)
    }
}

mod handlers {
    use super::{DiscordBotListWebhookBody, TopGgWebhookBody};
    use crate::assyst::Assyst;
    use std::sync::Arc;
    use warp::{Rejection, Reply};

    pub async fn root() -> Result<impl Reply, Rejection> {
        Ok(warp::reply::reply())
    }

    pub async fn dbl(
        body: DiscordBotListWebhookBody,
        assyst: Arc<Assyst>,
    ) -> Result<impl Reply, Rejection> {
        super::handle_vote(
            assyst.clone(),
            body.id.parse().unwrap(),
            "discordbotlist.com",
        )
        .await;

        Ok(warp::reply::reply())
    }

    pub async fn topgg(
        body: TopGgWebhookBody,
        assyst: Arc<Assyst>,
    ) -> Result<impl Reply, Rejection> {
        super::handle_vote(assyst.clone(), body.user.parse().unwrap(), "top.gg").await;

        Ok(warp::reply::reply())
    }
}

pub fn run(assyst: Arc<Assyst>) {
    use filters::*;
    use warp::{serve, Filter};

    let filters = root().or(dbl(assyst.clone())).or(topgg(assyst.clone()));
    tokio::spawn(async move {
        serve(filters)
            .run(([0, 0, 0, 0], assyst.config.bot_list_port))
            .await;
    });
}
