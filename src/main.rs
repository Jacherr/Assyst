mod command;
mod database;
mod handler;
mod handlers;
mod util;
mod assyst;
mod rest;
mod caching;

use dotenv::dotenv;
use futures::stream::StreamExt;
use handler::handle_event;
use std::env;
use std::sync::Arc;
use twilight_gateway::cluster::{Cluster, ShardScheme};
use twilight_model::gateway::Intents;
use assyst::Assyst;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").unwrap();

    // spawn as many shards as discord recommends
    let scheme = ShardScheme::Auto;
    let cluster = Cluster::builder(
        &token,
        Intents::GUILD_MESSAGES | Intents::GUILD_MESSAGE_REACTIONS,
    )
    .shard_scheme(scheme)
    .build()
    .await
    .unwrap();

    let spawned_cluster = cluster.clone();
    tokio::spawn(async move { spawned_cluster.up().await });

    let assyst = Arc::new(Assyst::new(&token).await);

    let mut events = cluster.events();

    while let Some((_, event)) = events.next().await {
        let assyst_clone = assyst.clone();
        tokio::spawn(async move { 
            handle_event(assyst_clone, event).await;
        });
    }
}
