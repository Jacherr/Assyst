use std::{sync::Arc, time::Duration};

use crate::{assyst::Assyst, logger, rest::post_bot_stats};

use tokio::time::sleep;

const FETCH_INTERVAL: u64 = 60000 * 60; // 1 hour

pub fn init_bot_list_posting_loop(assyst: Arc<Assyst>) {
    tokio::spawn(async move {
        loop {
            sleep(Duration::from_millis(FETCH_INTERVAL)).await;

            let guild_count = assyst.metrics.get_guild_count();

            if guild_count < 1000 { break; }

            let result = post_bot_stats(
                &assyst.reqwest_client,
                &assyst.config.auth.discord_bot_list_post_stats,
                &assyst.config.auth.top_gg_post_stats,
                &assyst.config.auth.discords_post_stats,
                guild_count as u32,
            )
            .await;

            match result {
                Ok(()) => {}
                Err(e) => {
                    logger::fatal(
                        &assyst,
                        &format!("Error POSTing bot list stats: {}", e.to_string()),
                    )
                    .await;
                }
            }
        }
    });
}
