#![allow(dead_code)]

mod assyst;
mod badtranslator;
mod botlist;
mod caching;
mod command;
mod config;
mod consts;
mod database;
mod filetype;
mod handler;
mod handlers;
mod logging;
mod metrics;
mod rest;
mod tasks;
mod util;

use assyst::Assyst;
use dotenv::dotenv;
use futures::stream::StreamExt;
use handler::handle_event;
use std::env;
use std::sync::Arc;
use twilight_gateway::cluster::{Cluster, ShardScheme};
use twilight_model::gateway::payload::update_presence::UpdatePresencePayload;
use twilight_model::gateway::{
    presence::{Activity, ActivityType, Status},
    Intents,
};
use botlist::run as botlist_run;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").unwrap();

    let assyst = Arc::new(Assyst::new(&token).await);
    let activity = Activity {
        application_id: None,
        assets: None,
        created_at: None,
        details: None,
        emoji: None,
        flags: None,
        id: None,
        instance: None,
        kind: ActivityType::Playing,
        name: format!("{}help | jacher.io/assyst", assyst.config.prefix.default),
        party: None,
        secrets: None,
        state: None,
        timestamps: None,
        url: None,
        buttons: Vec::new(),
    };
    let presence = UpdatePresencePayload::new(vec![activity], false, None, Status::Online).unwrap();

    // spawn as many shards as discord recommends
    let scheme = ShardScheme::Auto;
    let (cluster, mut events) = Cluster::builder(&token, Intents::GUILD_MESSAGES | Intents::GUILDS)
        .shard_scheme(scheme)
        .http_client(assyst.http.clone())
        .presence(presence)
        .build()
        .await
        .unwrap();

    let spawned_cluster = cluster.clone();
    tokio::spawn(async move { spawned_cluster.up().await });

    // Tasks
    tasks::init_bot_list_posting_loop(assyst.clone());
    tasks::init_reminder_loop(assyst.clone());
    tasks::init_caching_gc_loop(assyst.clone());
    tasks::update_patrons(assyst.clone());

    // Bot list webhooks
    botlist_run(assyst.clone());

    while let Some((_, event)) = events.next().await {
        assyst.metrics.write().await.processing.add_event();
        let assyst_clone = assyst.clone();
        tokio::spawn(async move {
            handle_event(assyst_clone, event).await;
        });
    }
}
