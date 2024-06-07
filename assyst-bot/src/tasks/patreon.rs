use std::{sync::Arc, time::Duration};

use serenity::all::Http;
use tokio::time::sleep;
use twilight_model::id::{marker::UserMarker, Id};

use crate::{assyst::Assyst, rest::patreon::{get_patrons, Patron}};

const FETCH_INTERVAL: i64 = 60000 * 60; // 1 hour

pub fn update_patrons(assyst: Arc<Assyst>) {
    tokio::spawn(async move {
        loop {
            let mut patrons = get_patrons(assyst.clone(), &assyst.config.auth.patreon)
                .await
                .unwrap();

            /*let patron_user_ids = patrons
            .iter()
            .filter(|x| !x.admin)
            .map(|x| x.user_id.get().to_string())
            .collect::<Vec<_>>()
            .join("\n");*/

            let serenity_http = Http::new(&assyst.config.auth.discord);
            let entitlements = serenity_http.get_entitlements(None, None, None, None, None, None, None).await.unwrap();
            for entitlement in entitlements {
                if entitlement.sku_id == 1248746470115381410 /*Assyst Premium T2 */ {
                    patrons.push(Patron {
                        user_id: Id::<UserMarker>::new(entitlement.user_id.map(|x| x.get()).unwrap_or(0)),
                        tier: 2,
                        admin: false
                    })
                }
            }


            *assyst.patrons.write().await = patrons;

            sleep(Duration::from_millis(FETCH_INTERVAL as u64)).await;
        }
    });
}
