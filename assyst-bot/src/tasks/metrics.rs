use std::sync::Arc;

use crate::{assyst::Assyst, logger, util};
use prometheus::{register_gauge, register_int_gauge_vec};
use std::time::Duration;
use tokio::time::sleep;
use twilight_gateway::{shard::Stage, Cluster};

pub fn init_metrics_collect_loop(cluster: Cluster, assyst: Arc<Assyst>) -> anyhow::Result<()> {
    let memory_counter = register_gauge!("memory_usage", "Memory usage in MB")?;
    let latency = register_int_gauge_vec!("latency", "Gateway latency", &["shard"])?;

    tokio::spawn(async move {
        loop {
            // gives time for shards to start before collecting info about them
            sleep(Duration::from_secs(150)).await;

            // collect memory usage
            match util::get_memory_usage_num() {
                Some(memory) => memory_counter.set(memory as f64),
                None => {
                    logger::fatal(
                        &assyst,
                        "Failed to scrape memory usage in metrics collector",
                    )
                    .await
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
                    logger::info(&assyst, &format!("Shard {} is starting", i)).await;
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
                        logger::fatal(&assyst, &format!("Failed to get shard info: {}", e)).await;
                        continue;
                    }
                };
            }
        }
    });

    Ok(())
}
