#![allow(dead_code)]

use std::sync::Arc;

use assyst_common::{config::Config, util::to_static_str};
use assyst_database::Database;
use filters::*;
use twilight_http::Client as HttpClient;
use warp::Filter;

mod filters;
mod handlers;
mod server;

#[rustfmt::skip]
pub fn run(config: Arc<Config>, database: Arc<Database>, client: HttpClient) {
    // TODO(y21): this can be done without leaking, but it's low priority because the webserver will run forever anyway
    let auth = to_static_str(&config.auth.bot_list_webhook);

    // This excessive cloning may look expensive, but it's not; these are all Arc clones, aka atomic integer increments
    let filters = root()
        .or(metrics())
        .or(dbl(config.clone(), database.clone(), client.clone(), auth))
        .or(topgg(config.clone(), database.clone(), client.clone(), auth))
        .or(dbl_redirect())
        .or(topgg_redirect());

    tokio::spawn(async move {
        warp::serve(filters)
            .run(([0, 0, 0, 0], config.bot_list_port))
            .await;
    });
}
