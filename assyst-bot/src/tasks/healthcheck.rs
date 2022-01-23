use std::{time::{Duration, Instant}, sync::Arc};

use tokio::time::sleep;

use crate::{rest::{healthcheck, ServiceStatus}, assyst::Assyst, logger};

pub fn init_healthcheck(assyst: Arc<Assyst>) {
    tokio::spawn(async move {
        loop {
            let result = healthcheck(assyst.clone()).await;

            for x in result.clone() {
                if x.status == ServiceStatus::Offline {
                    logger::fatal(&assyst, &format!("{} is offline", x.service)).await;
                }
            }

            *assyst.healthcheck_result.lock().await =
                (Instant::now(), result);
        
            sleep(Duration::from_secs(5 * 60)).await;
        }
    });
}
