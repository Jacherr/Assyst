#![allow(dead_code)]

// a

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
use assyst_common::consts::{
    gateway::{self, Latencies},
    EVENT_PIPE,
};
use assyst_webserver::run as webserver_run;
use bincode::deserialize;
use caching::persistent_caching::get_guild_count;
use handler::handle_event;
use serde::de::DeserializeSeed;
use std::sync::Arc;
use tokio::{
    io::{AsyncReadExt, BufReader},
    net::UnixStream,
};
use twilight_model::gateway::event::GatewayEventDeserializer;

#[cfg(target_os = "linux")]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let assyst = Arc::new(Assyst::new().await);

    // Tasks
    tasks::init_bot_list_posting_loop(assyst.clone());
    tasks::init_reminder_loop(assyst.clone());
    tasks::init_caching_gc_loop(assyst.clone());
    tasks::update_patrons(assyst.clone());
    tasks::init_healthcheck(assyst.clone());
    tasks::init_metrics_collect_loop(assyst.clone())
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

    match get_guild_count(assyst.clone()).await {
        Ok(g) => assyst.metrics.add_guilds(g as i64),
        Err(_) => logger::fatal(assyst.as_ref(), "failed to get guild count").await,
    };

    assyst.initialize_blacklist().await?;

    let stream = UnixStream::connect(EVENT_PIPE).await?;
    let mut reader = BufReader::new(stream);

    // Event loop
    loop {
        let assyst_clone = assyst.clone();
        let op = reader.read_u8().await?;
        let len = reader.read_u32().await?;
        let mut data: Vec<u8> = vec![0; len as usize];
        reader.read_exact(&mut data).await?;
        match op {
            gateway::OP_EVENT => {
                assyst.metrics.add_event();
                tokio::spawn(async move {
                    let json = String::from_utf8_lossy(&data);
                    let de = GatewayEventDeserializer::from_json(&json).unwrap();
                    let mut json_de = serde_json::Deserializer::from_str(&json);
                    match de.deserialize(&mut json_de) {
                        Ok(x) => {
                            let res = handle_event(assyst_clone.clone(), x).await;
                            match res {
                                Err(e) => {
                                    logger::fatal(
                                        assyst_clone.as_ref(),
                                        &format!("Event error: {}", e.to_string()),
                                    )
                                    .await;
                                }
                                _ => {}
                            }
                        }
                        Err(_) => {}
                    };
                });
            }
            gateway::OP_LATENCIES => {
                let deserialized: Latencies = match deserialize(&data) {
                    Ok(d) => d,
                    Err(e) => {
                        logger::fatal(
                            assyst.as_ref(),
                            &format!(
                                "Failed to deserialize latency information: {}",
                                e.to_string()
                            ),
                        )
                        .await;
                        continue;
                    }
                };

                for latency in deserialized.0 {
                    assyst.metrics.set_shard_latency(latency.0, latency.1);
                }
            }
            _ => {}
        }
    }
}
