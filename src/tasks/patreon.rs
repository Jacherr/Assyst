use std::{sync::Arc, time::Duration};

use tokio::time::sleep;

use crate::{assyst::Assyst, rest::patreon::get_patrons};

const FETCH_INTERVAL: i64 = 60000 * 60; // 1 hour

pub fn update_patrons(assyst: Arc<Assyst>) {
    tokio::spawn(async move {
        *assyst.patrons.write().await = get_patrons(assyst.clone(), &assyst.config.auth.patreon)
            .await
            .unwrap();
        sleep(Duration::from_millis(FETCH_INTERVAL as u64)).await;
    });
}
