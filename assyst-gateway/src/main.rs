#![feature(never_type)]

use std::{collections::HashMap, sync::Arc, time::Duration, rc::Rc};

use assyst_common::{
    config::Config,
    consts::{
        gateway::{self, Latencies},
        EVENT_PIPE,
    },
    ok_or_break
};
use bincode::serialize;
use futures_util::StreamExt;
use tokio::{
    io::{AsyncWriteExt, BufWriter},
    net::UnixListener,
    sync::Mutex,
    time::sleep, fs::remove_file,
};
use twilight_gateway::{cluster::Events, Cluster, Event, EventTypeFlags, Intents};
use twilight_model::gateway::{
    payload::outgoing::update_presence::UpdatePresencePayload,
    presence::{Activity, ActivityType, Status},
};

#[tokio::main]
async fn main() -> anyhow::Result<!> {
    let _ = remove_file(EVENT_PIPE).await;

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
    let (cluster, events) = Cluster::builder(
        config.auth.discord.to_string(),
        Intents::MESSAGE_CONTENT | Intents::GUILD_MESSAGES | Intents::GUILDS,
    )
    .presence(presence)
    .event_types(EventTypeFlags::SHARD_PAYLOAD)
    .build()
    .await?;

    let events = Arc::new(Mutex::new(events));
    let cluster = Arc::new(cluster);

    let spawned_cluster = cluster.clone();
    tokio::spawn(async move {
        spawned_cluster.up().await;
    });

    let listener = UnixListener::bind(EVENT_PIPE)?;
    let listener = Rc::new(listener);

    loop {
        let _ = supply_connection(listener.clone(), events.clone(), cluster.clone()).await;
    }
}

pub async fn supply_connection(
    listener: Rc<UnixListener>,
    events: Arc<Mutex<Events>>,
    cluster: Arc<Cluster>,
) -> anyhow::Result<()> {
    let (stream, _) = listener.accept().await?;
    let writer = Arc::new(Mutex::new(BufWriter::new(stream)));
    let writer_clone = writer.clone();

    let latency_writer = tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(5)).await;
            let mut latencies = Latencies(HashMap::new());
            let info = cluster.info();
            for shard in info {
                let latency = match shard.1.latency().average().map(|d| d.as_millis()) {
                    Some(x) => x as i64,
                    None => continue,
                };
                if latency > 0 {
                    latencies.0.insert(shard.0, latency);
                }
            }
            let serialized = serialize(&latencies).unwrap();
            let mut lock = writer_clone.lock().await;
            ok_or_break!(lock.write_u8(gateway::OP_LATENCIES).await);
            ok_or_break!(lock.write_u32(serialized.len() as u32).await);
            ok_or_break!(lock.write_all(&serialized).await);
            ok_or_break!(lock.flush().await);
        }
    });

    // Event loop
    while let Some((_, event)) = events.lock().await.next().await {
        match event {
            Event::ShardPayload(x) => {
                let mut lock = writer.lock().await;
                ok_or_break!(lock.write_u8(gateway::OP_EVENT).await);
                ok_or_break!(lock.write_u32(x.bytes.len() as u32).await);
                ok_or_break!(lock.write_all(&x.bytes).await);
                ok_or_break!(lock.flush().await);
            }
            o => {
                println!("{:?}", o);
            }
        }
    }

    latency_writer.abort();

    Ok(())
}
