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
        let user = assyst.http.user(UserId::from(user_id as u64)).await.unwrap();
        let message;
        let user_votes_entry = assyst.database.get_voter(user_id).await;
        let user_votes = if let Some(u) = user_votes_entry {
            u.count
        } else {
            0
        };

        match user {
            Some(u) => {
                assyst
                    .database
                    .increment_user_votes(user_id, &u.name, &u.discriminator)
                    .await;

                message = format!(
                    "{0}#{1} voted for Assyst on {2} and got {3} free tier 1 requests!\n{0}#{1} has voted {4} total times.",
                    u.name, u.discriminator, service, VOTE_FREE_TIER_1_REQUESTS, user_votes
                )
            }
            None => {
                message = format!(
                    "An unknown user voted for Assyst on {} and got {} free tier 1 requests!",
                    service, VOTE_FREE_TIER_1_REQUESTS
                )
            }
        }

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
            "[discordbotlist.com](https://discordbotlist.com/bots/assyst/upvote)",
        )
        .await;

        Ok(warp::reply::reply())
    }

    pub async fn topgg(
        body: TopGgWebhookBody,
        assyst: Arc<Assyst>,
    ) -> Result<impl Reply, Rejection> {
        super::handle_vote(
            assyst.clone(),
            body.user.parse().unwrap(),
            "[top.gg](https://top.gg/bot/571661221854707713/vote)",
        )
        .await;

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
