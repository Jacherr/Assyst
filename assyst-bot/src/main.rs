#![allow(dead_code)]

mod assyst;
mod badtranslator;
mod caching;
mod command;
// mod consts;
mod filetype;
mod handler;
mod handlers;
mod logger;
mod metrics;
mod rest;
mod tasks;
mod util;
mod ansi;

use assyst::Assyst;
use assyst_webserver::run as webserver_run;
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

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

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
    tasks::init_metrics_collect_loop(cluster, assyst.clone())
        .expect("Failed to initialize metrics collect loop");

    // Bot list webhooks and metrics
    webserver_run(
        assyst.config.clone(),
        assyst.database.clone(),
        assyst.http.clone(),
    );

    // Custom panic hook that will send errors to a discord channel
    {
        let handle = tokio::runtime::Handle::current();
        let assyst = Arc::clone(&assyst);

        std::panic::set_hook(Box::new(move |info| {
            println!("{}", info);

            let assyst = assyst.clone();
            let msg = format!("a thread has panicked: {}", info);

            handle.spawn(async move { logger::panic(&assyst, &msg).await });
        }));
    }

    if assyst.badtranslator.should_fetch().await {
        assyst.initialize_bt().await;
    }

    // Event loop
    while let Some((_, event)) = events.next().await {
        assyst.metrics.add_event();
        let assyst_clone = assyst.clone();
        tokio::spawn(async move {
            handle_event(assyst_clone, event).await;
        });
    }

    println!("{}", "shutting down");
}
