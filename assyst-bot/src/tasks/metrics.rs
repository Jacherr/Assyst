use std::sync::Arc;

use crate::{assyst::Assyst, logger, rest::ServiceStatus, util};
use prometheus::{register_gauge, register_int_gauge_vec};
use std::time::Duration;
use tokio::time::sleep;
use twilight_gateway::{shard::Stage, Cluster};

pub fn init_metrics_collect_loop(cluster: Arc<Cluster>, assyst: Arc<Assyst>) -> anyhow::Result<()> {
    let memory_counter = register_gauge!("memory_usage", "Memory usage in MB")?;
    let latency = register_int_gauge_vec!("latency", "Gateway latency", &["shard"])?;
    let health = register_int_gauge_vec!("service_ping", "Service ping", &["service"])?;
    let commands_usage = register_int_gauge_vec!("commands_usage", "Commands usage", &["command"])?;
    let cache_size = register_int_gauge_vec!("cache_sizes", "Cache sizes", &["cache"])?;

    let a = assyst.clone();

    tokio::spawn(async move {
        loop {
            // gives time for shards to start before collecting info about them
            sleep(Duration::from_secs(60)).await;

            // collect memory usage
            match util::get_memory_usage_num() {
                Some(memory) => memory_counter.set(memory as f64),
                None => {
                    logger::fatal(&a, "Failed to scrape memory usage in metrics collector").await
                }
            };

            let mut up_shards = Vec::<u64>::new();
            let cluster_info = cluster.info();
            for shard in cluster_info {
                if shard.1.stage() == Stage::Connected {
                    up_shards.push(shard.0);
                }
            }

            // collect latency of each shard
            let mut i: u64 = 0;
            for shard in cluster.shards() {
                if !up_shards.contains(&i) {
                    logger::info(&a, &format!("Shard {} is starting", i)).await;
                    shard.start().await.unwrap();
                    // wait to avoid spamming identifies
                    tokio::time::sleep(Duration::from_secs(10)).await;
                }
                i += 1;

                match shard.info() {
                    Ok(info) => {
                        let lat = match info.latency().average().map(|d| d.as_millis()) {
                            Some(x) => x as i64,
                            None => continue,
                        };

                        let id = info.id().to_string();

                        let counter = latency.with_label_values(&[&id]);
                        counter.set(lat);
                    }
                    Err(e) => {
                        logger::fatal(&a, &format!("Failed to get shard info: {}", e)).await;
                        continue;
                    }
                };
            }

            let healthcheck_result = &a.healthcheck_result.lock().await.1;
            for result in healthcheck_result {
                let counter = health.with_label_values(&[&result.service]);
                if let ServiceStatus::Online(x) = result.status {
                    counter.set(x as i64);
                } else {
                    counter.set(-100);
                }
            }
        }
    });

    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(10)).await;

            let top_commands = assyst.database.get_command_usage_stats().await.unwrap();
            for command in top_commands {
                let counter = commands_usage.with_label_values(&[&command.command_name]);
                counter.set(command.uses as i64);
            }

            let replies_size = assyst.replies.read().await.size();
            let ratelimits_size = assyst.command_ratelimits.read().await.size();
            let prefixes_size = assyst.database.cache.read().await.prefixes.keys().len();
            let disabled_commands_size = assyst
                .database
                .cache
                .read()
                .await
                .disabled_commands
                .cache
                .keys()
                .len();
            let counter = cache_size.with_label_values(&["replies"]);
            counter.set(replies_size as i64);
            let counter = cache_size.with_label_values(&["ratelimits"]);
            counter.set(ratelimits_size as i64);
            let counter = cache_size.with_label_values(&["prefixes"]);
            counter.set(prefixes_size as i64);
            let counter = cache_size.with_label_values(&["disabled_commands"]);
            counter.set(disabled_commands_size as i64);
        }
    });

    Ok(())
}
