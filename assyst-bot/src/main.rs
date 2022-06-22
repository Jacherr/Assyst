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

use assyst::Assyst;
use assyst_common::consts::EVENT_PIPE;
use assyst_webserver::run as webserver_run;
use handler::handle_event;
use libc::mkfifo;
use serde::de::DeserializeSeed;
use tokio::fs::read;
use twilight_model::gateway::event::GatewayEventDeserializer;
use std::{sync::Arc, ffi::CString};

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
    //tasks::init_metrics_collect_loop(arced_cluster.clone(), assyst.clone())
    //    .context("Failed to initialize metrics collect loop")?;

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
    loop {
        let assyst_clone = assyst.clone();
        let d = read(EVENT_PIPE).await?;
        tokio::spawn(async move {
            let json = String::from_utf8_lossy(&d);
            let de = GatewayEventDeserializer::from_json(&json).unwrap();
            let mut json_de = serde_json::Deserializer::from_str(&json);
            match de.deserialize(&mut json_de) {
                Ok(x) => {
                    handle_event(assyst_clone, x).await;
                }
                Err(_) => {}
            };
        });
        assyst.metrics.add_event();
    }
}
