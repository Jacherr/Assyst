mod command;
mod database;
mod handler;
mod handlers;

use command::client;
use client::CommandClient;
use database::Database;
use std::{sync::Arc, fs::read_to_string};
use futures::stream::StreamExt;
use handler::handle_event;
use twilight_gateway::cluster::{Cluster, ShardScheme};
use twilight_http::Client as HttpClient;
use twilight_model::gateway::Intents;
use serde::Deserialize;
use dotenv::dotenv;
use std::env;

#[derive(Clone, Deserialize)]
struct DatabaseInfo {
    username: Box<str>,
    password: Box<str>,
    host: Box<str>,
    port: u16,
    database: Box<str>
}
impl DatabaseInfo {
    pub fn to_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password,
            self.host,
            self.port,
            self.database
        )
    }
}
#[derive(Clone, Deserialize)]
struct Config {
    database: DatabaseInfo,
    default_prefix: Box<str>
}
impl Config {
    fn new() -> Self {
        let buffer = read_to_string("./config.toml").unwrap();
        toml::from_str(&buffer).unwrap()
    }
}
pub struct Assyst {
    command_client: CommandClient,
    config: Config,
    database: Database,
#[allow(dead_code)]
    http: HttpClient
}

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

    let http = HttpClient::new(&token);
    let config = Config::new();
    let database = database::Database::new(
        2,
        config.database.to_url()
    )
    .await
    .unwrap();

    let assyst = Arc::new(Assyst {
        command_client: CommandClient::new(),
        config,
        database,
        http
    });

    let mut events = cluster.events();
    while let Some((_, event)) = events.next().await {
        tokio::spawn(handle_event(assyst.clone(), event));
    }
}