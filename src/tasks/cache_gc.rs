use std::{sync::Arc, time::Duration};

use tokio::time::sleep;

use crate::assyst::Assyst;

const FETCH_INTERVAL: i64 = 30000;

pub fn init_caching_gc_loop(assyst: Arc<Assyst>) {
    tokio::spawn(async move {
        assyst.replies.write().await.garbage_collect().await;
        sleep(Duration::from_millis(FETCH_INTERVAL as u64)).await;
    });
}
