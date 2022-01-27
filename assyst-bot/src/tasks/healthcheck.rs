use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use tokio::time::sleep;

use crate::{
    assyst::Assyst,
    logger,
    rest::{healthcheck, ServiceStatus},
};

pub fn init_healthcheck(assyst: Arc<Assyst>) {
    tokio::spawn(async move {
        loop {
            let result = healthcheck(assyst.clone()).await;

            for check in &result {
                if check.status == ServiceStatus::Offline {
                    logger::fatal(&assyst, &format!("{} is offline", check.service)).await;
                }
            }

            *assyst.healthcheck_result.lock().await = (Instant::now(), result);

            sleep(Duration::from_secs(5 * 60)).await;
        }
    });
}
