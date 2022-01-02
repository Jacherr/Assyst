use super::handlers;
use assyst_common::config::Config;
use assyst_database::Database;
use std::{convert::Infallible, sync::Arc};
use twilight_http::Client as HttpClient;
use warp::{Filter, Rejection, Reply};

const DISCORD_BOT_LIST_ENDPOINT: &str = "dbl";
const TOP_GG_ENDPOINT: &str = "topgg";
const METRICS_ENDPOINT: &str = "metrics";

pub fn root() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path::end().and(warp::get()).and_then(handlers::root)
}

pub fn parts(
    config: Arc<Config>,
    database: Arc<Database>,
    client: HttpClient,
) -> impl Filter<Extract = (Arc<Config>, Arc<Database>, HttpClient), Error = Infallible> + Clone {
    warp::any()
        .map(move || config.clone())
        .and(warp::any().map(move || database.clone()))
        .and(warp::any().map(move || client.clone()))
}

pub fn dbl_redirect() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path(DISCORD_BOT_LIST_ENDPOINT)
        .and(warp::get())
        .and_then(handlers::dbl_redirect)
        .boxed()
}

pub fn dbl(
    config: Arc<Config>,
    database: Arc<Database>,
    client: HttpClient,
    auth: &'static str,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path(DISCORD_BOT_LIST_ENDPOINT)
        .and(warp::post())
        .and(warp::header::exact("authorization", auth))
        .and(warp::body::json())
        .and(parts(config, database, client))
        .and_then(handlers::dbl)
        .boxed()
}

pub fn topgg_redirect() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path(TOP_GG_ENDPOINT)
        .and(warp::get())
        .and_then(handlers::topgg_redirect)
        .boxed()
}

pub fn topgg(
    config: Arc<Config>,
    database: Arc<Database>,
    client: HttpClient,
    auth: &'static str,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path(TOP_GG_ENDPOINT)
        .and(warp::post())
        .and(warp::header::exact("authorization", auth))
        .and(warp::body::json())
        .and(parts(config, database, client))
        .and_then(handlers::topgg)
        .boxed()
}

pub fn metrics() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path(METRICS_ENDPOINT)
        .and(warp::get())
        .and_then(handlers::metrics)
        .boxed()
}
