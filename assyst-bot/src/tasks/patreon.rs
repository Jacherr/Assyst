use std::{sync::Arc, time::Duration};

use crate::util::ChannelId;
use tokio::time::sleep;

use crate::{assyst::Assyst, rest::patreon::get_patrons};

const FETCH_INTERVAL: i64 = 60000 * 60; // 1 hour

pub fn update_patrons(assyst: Arc<Assyst>) {
    tokio::spawn(async move {
        loop {
            let patrons = get_patrons(assyst.clone(), &assyst.config.auth.patreon)
                .await
                .unwrap();

            assyst
                .http
                .create_message(ChannelId::new(833758252427640892))
                .content(&format!(
                    "patrons:\n{}",
                    patrons
                        .iter()
                        .filter(|x| !x.admin)
                        .map(|x| x.user_id.get().to_string())
                        .collect::<Vec<_>>()
                        .join("\n")
                ))
                .unwrap()
                .exec()
                .await
                .unwrap();

            *assyst.patrons.write().await = patrons;

            sleep(Duration::from_millis(FETCH_INTERVAL as u64)).await;
        }
    });
}
