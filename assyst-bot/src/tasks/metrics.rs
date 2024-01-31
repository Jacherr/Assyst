use std::sync::Arc;

use crate::{
    assyst::Assyst,
    logger,
    rest::{get_filer_stats, FilerStats, ServiceStatus},
    util,
};
use prometheus::{register_gauge, register_int_gauge_vec};
use std::time::Duration;
use std::time::Instant;
use tokio::time::sleep;

static COMMAND_RATE_SAMPLE_TIME: u64 = 60 * 60;

pub fn init_metrics_collect_loop(assyst: Arc<Assyst>) -> anyhow::Result<()> {
    let memory_counter = register_gauge!("memory_usage", "Memory usage in MB")?;
    let health = register_int_gauge_vec!("service_ping", "Service ping", &["service"])?;
    let commands_usage = register_int_gauge_vec!("commands_usage", "Commands usage", &["command"])?;
    let cache_size = register_int_gauge_vec!("cache_sizes", "Cache sizes", &["cache"])?;

    let a = assyst.clone();
    let a2 = assyst.clone();

    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(60 * 1)).await;
            let command_uses = a2.database.get_command_usage_stats().await.unwrap();

            let mut diff = a2.command_usage_diff.lock().await;
            for i in command_uses {
                if let Some(diff) = diff.iter_mut().find(|d| d.0 == i.command_name) {
                    diff.1.push((i.uses as usize, Instant::now()));
                    let mut to_remove = vec![];
                    for (pos, entry) in diff.1.iter().enumerate() {
                        // remove old entries
                        if Instant::now().duration_since(entry.1).as_secs()
                            > COMMAND_RATE_SAMPLE_TIME
                        {
                            to_remove.push(pos);
                        }
                    }
                    for i in to_remove {
                        diff.1.remove(i);
                    }
                    diff.1.sort_by(|a, b| a.0.cmp(&b.0));
                } else {
                    diff.push((i.command_name, vec![(i.uses as usize, Instant::now())]));
                }
            }
        }
    });

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

            let filer_stats = get_filer_stats(assyst.clone()).await.unwrap_or(FilerStats {
                count: 0,
                size_bytes: 0,
            });
            assyst.metrics.set_cdn_files(filer_stats.count as i64);
            assyst.metrics.set_cdn_size(filer_stats.size_bytes as i64);
        }
    });

    Ok(())
}
