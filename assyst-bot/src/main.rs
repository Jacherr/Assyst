#![allow(dead_code)]

mod ansi;
mod assyst;
mod badtranslator;
mod caching;
mod command;
mod downloader;
mod handler;
mod handlers;
mod logger;
mod metrics;
mod rest;
mod tasks;
mod util;

use anyhow::Context;
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

#[cfg(target_os = "linux")]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN")?;

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
    let presence = UpdatePresencePayload::new(vec![activity], false, None, Status::Online)?;

    // spawn as many shards as discord recommends
    let scheme = ShardScheme::Auto;
    let (cluster, mut events) = Cluster::builder(&token, Intents::GUILD_MESSAGES | Intents::GUILDS)
        .shard_scheme(scheme)
        .http_client(assyst.http.clone())
        .presence(presence)
        .build()
        .await?;

    let spawned_cluster = cluster.clone();
    let a = assyst.clone();
    tokio::spawn(async move {
        spawned_cluster.up().await;
        let cluster_info = spawned_cluster.info();
        let mut text = String::new();
        for shard in cluster_info {
            text.push_str(&format!("Shard {} is {}\n", shard.0, shard.1.stage()));
        }
        logger::info(&a, &text).await;
    });

    // Tasks
    tasks::init_bot_list_posting_loop(assyst.clone());
    tasks::init_reminder_loop(assyst.clone());
    tasks::init_caching_gc_loop(assyst.clone());
    tasks::update_patrons(assyst.clone());
    tasks::init_healthcheck(assyst.clone());
    tasks::init_metrics_collect_loop(cluster, assyst.clone())
        .context("Failed to initialize metrics collect loop")?;

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

    assyst.initialize_blacklist().await?;

    // Event loop
    while let Some((_, event)) = events.next().await {
        let assyst_clone = assyst.clone();
        tokio::spawn(async move {
            handle_event(assyst_clone, event).await;
        });
        assyst.metrics.add_event();
    }

    println!("{}", "shutting down");

    Ok(())
}
