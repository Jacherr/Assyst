use assyst_common::config::Config;
use assyst_database::Database;
use assyst_logger as logger;
use prometheus::TextEncoder;
use serde::Deserialize;
use std::sync::Arc;
use twilight_http::Client as HttpClient;
use twilight_model::id::{marker::UserMarker, Id};
use warp::{hyper::Uri, Rejection, Reply};

type UserId = Id<UserMarker>;

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

pub async fn handle_vote(
    config: &Config,
    database: &Database,
    client: &Arc<HttpClient>,
    user_id: i64,
    service: &str,
) {
    let result = database
        .add_free_tier_1_requests(user_id, VOTE_FREE_TIER_1_REQUESTS)
        .await;

    if let Err(e) = result {
        logger::fatal(
            config,
            database,
            &format!(
                "failed to give free tier 1 requests to voter, reason: {}",
                e.to_string()
            ),
        )
        .await;
    } else {
        let user = client
            .user(UserId::new(user_id as u64))
            .await
            .unwrap()
            .model()
            .await
            .unwrap();

        let message;
        database
            .increment_user_votes(user_id, &user.name, &user.discriminator.to_string())
            .await;

        let user_votes_entry = database.get_voter(user_id).await;
        let user_votes = if let Some(u) = user_votes_entry {
            u.count
        } else {
            0
        };

        message = format!(
                    "{0}#{1} voted for Assyst on {2} and got {3} free tier 1 requests!\n{0}#{1} has voted {4} total times.",
                    user.name, user.discriminator, service, VOTE_FREE_TIER_1_REQUESTS, user_votes
                );

        logger::log_vote(config, client, &message).await;
    }
}

pub async fn root() -> Result<impl Reply, Rejection> {
    Ok(warp::redirect::redirect(Uri::from_static(
        "https://jacher.io/assyst",
    )))
}

pub async fn metrics() -> Result<impl warp::Reply, Rejection> {
    let encoder = TextEncoder::new();
    let family = prometheus::gather();
    let response = encoder.encode_to_string(&family).expect("Encoding failed");
    Ok(response)
}

pub async fn dbl_redirect() -> Result<impl Reply, Rejection> {
    Ok(warp::redirect::redirect(Uri::from_static(
        "https://discordbotlist.com/bots/assyst/upvote",
    )))
}

pub async fn dbl(
    body: DiscordBotListWebhookBody,
    config: Arc<Config>,
    database: Arc<Database>,
    client: Arc<HttpClient>,
) -> Result<impl Reply, Rejection> {
    handle_vote(
        &config,
        &database,
        &client,
        body.id.parse().unwrap(),
        "[discordbotlist.com](https://discordbotlist.com/bots/assyst/upvote)",
    )
    .await;

    Ok(warp::reply::reply())
}

pub async fn topgg_redirect() -> Result<impl Reply, Rejection> {
    Ok(warp::redirect::redirect(Uri::from_static(
        "https://top.gg/bot/571661221854707713/vote",
    )))
}

pub async fn topgg(
    body: TopGgWebhookBody,
    config: Arc<Config>,
    database: Arc<Database>,
    client: Arc<HttpClient>,
) -> Result<impl Reply, Rejection> {
    handle_vote(
        &config,
        &database,
        &client,
        body.user.parse().unwrap(),
        "[top.gg](https://top.gg/bot/571661221854707713/vote)",
    )
    .await;

    Ok(warp::reply::reply())
}
