use std::{ffi::CString, sync::Arc};

use assyst_common::{config::Config, consts::EVENT_PIPE};
use libc::mkfifo;
use twilight_gateway::{Cluster, Intents, EventTypeFlags, Event};
use twilight_model::gateway::{
    payload::outgoing::update_presence::UpdatePresencePayload,
    presence::{Activity, ActivityType, Status},
};
use futures_util::StreamExt;
use tokio::fs::write;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::new();

    let activity = Activity {
        application_id: None,
        assets: None,
        created_at: None,
        details: None,
        emoji: None,
        flags: None,
        id: None,
        instance: None,
        kind: ActivityType::Playing,
        name: format!("{}help | jacher.io/assyst", config.prefix.default),
        party: None,
        secrets: None,
        state: None,
        timestamps: None,
        url: None,
        buttons: Vec::new(),
    };
    let presence = UpdatePresencePayload::new(vec![activity], false, None, Status::Online)?;

    // spawn as many shards as discord recommends
    let (cluster, mut events) = Cluster::builder(
        config.auth.discord.to_string(),
        Intents::MESSAGE_CONTENT | Intents::GUILD_MESSAGES | Intents::GUILDS,
    )
    .presence(presence)
    .event_types(EventTypeFlags::SHARD_PAYLOAD)
    .build()
    .await?;

    let arced_cluster = Arc::new(cluster);

    let spawned_cluster = arced_cluster.clone();
    tokio::spawn(async move {
        spawned_cluster.up().await;
    });

    unsafe {
        let path = CString::new(EVENT_PIPE).unwrap();
        mkfifo(path.as_ptr(), 0o644);
    }

    // Event loop
    while let Some((_, event)) = events.next().await {
        match event {
            Event::ShardPayload(x) => {
                write(EVENT_PIPE, &x.bytes).await?; 
            },
            o => {
                println!("{:?}", o);
            }
        }
    }

    Ok(())
}
