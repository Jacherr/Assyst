use std::{error::Error, sync::Arc};

use crate::{assyst::Assyst, logger, util};
use prometheus::{register_gauge, register_int_gauge_vec};
use std::time::Duration;
use tokio::time::sleep;
use twilight_gateway::Cluster;

pub fn init_metrics_collect_loop(
    cluster: Cluster,
    assyst: Arc<Assyst>,
) -> Result<(), Box<dyn Error>> {
    let memory_counter = register_gauge!("memory_usage", "Memory usage in MB")?;
    let latency = register_int_gauge_vec!("latency", "Gateway latency", &["shard"])?;

    tokio::spawn(async move {
        loop {
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

            // collect latency of each shard
            for shard in cluster.shards() {
                match shard.info() {
                    Ok(info) => {
                        let lat =
                            info.latency().average().map(|d| d.as_millis()).unwrap_or(0) as i64;

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

            sleep(Duration::from_secs(10)).await;
        }
    });

    Ok(())
}
