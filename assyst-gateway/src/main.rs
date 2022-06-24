use std::sync::Arc;

use assyst_common::{config::Config, consts::{EVENT_PIPE, gateway}};
use twilight_gateway::{Cluster, Intents, EventTypeFlags, Event, shard::Stage};
use twilight_model::gateway::{
    payload::outgoing::update_presence::UpdatePresencePayload,
    presence::{Activity, ActivityType, Status},
};
use futures_util::StreamExt;
use tokio::{net::UnixListener, io::{AsyncWriteExt, AsyncReadExt}, fs::remove_file, sync::Mutex};

macro_rules! ok_or_break {
    ($expression:expr) => {
      match $expression {
        Ok(v) => v,
        Err(_) break
      }
    }
  }

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    remove_file(EVENT_PIPE).await?;

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

    let listener = UnixListener::bind(EVENT_PIPE)?;

    let arced_events = Arc::new(Mutex::new(events));

    loop {
        let (mut stream, _) = listener.accept().await?;

        let (mut r, mut w) = stream.into_split();

        let info = arced_cluster.info();
        let mut up_shards: Vec<u64> = vec![];
        for shard in info {
            if shard.1.stage() == Stage::Connected {
                up_shards.push(shard.0)
            }
        }
        let shard_string = up_shards.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        w.write_u8(gateway::GATEWAY_OP_UP_SHARD).await?;
        w.write_u32(shard_string.len() as u32).await?;
        w.write_all(&shard_string.as_bytes()).await?;

        let events_clone = arced_events.clone();
        tokio::spawn(async move {
            while let Some((_, event)) = events_clone.lock().await.next().await {
                match event {
                    Event::ShardPayload(x) => {
                        ok_or_break!(w.write_u8(gateway::GATEWAY_OP_EVENT).await);
                        ok_or_break!(w.write_u32(x.bytes.len() as u32).await);
                        ok_or_break!(w.write_all(&x.bytes).await)
                    },
                    o => {
                        println!("{:?}", o);
                    }
                }
            }
        });

        tokio::spawn(async move {
            loop {
                let op = ok_or_break!(r.read_u8().await);
                let len = ok_or_break!(r.read_u32().await);
                let mut data = vec![0; len as usize];
                ok_or_break!(r.read_exact(&mut data).await);
                
            }
        });
    }
}
