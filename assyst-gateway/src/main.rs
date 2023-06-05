#![feature(never_type)]

use std::{collections::HashMap, sync::Arc, time::{Duration, SystemTime}, rc::Rc};

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
use twilight_gateway::{stream::{create_recommended, ShardMessageStream}, Intents, Config as GatewayConfig, Message};
use twilight_model::gateway::{
    payload::outgoing::update_presence::UpdatePresencePayload,
    presence::{Activity, ActivityType, Status},
};
use twilight_http::Client;

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
    let http_client = Client::new(config.auth.discord.to_string());
    let gateway_config = GatewayConfig::builder(config.auth.discord.to_string(), Intents::GUILD_MESSAGES | Intents::GUILDS | Intents::MESSAGE_CONTENT).presence(presence).build();

    let mut shards = create_recommended(&http_client, gateway_config.clone(), |_, _| gateway_config.clone()).await?.collect::<Vec<_>>();
    let stream = Arc::new(Mutex::new(ShardMessageStream::new(shards.iter_mut())));

    let listener = UnixListener::bind(EVENT_PIPE)?;
    let listener = Rc::new(listener);

    loop {
        let _ = supply_connection(listener.clone(), stream.clone()).await;
    }
}

pub async fn supply_connection(
    listener: Rc<UnixListener>,
    gw_stream: Arc<Mutex<ShardMessageStream<'_>>>
) -> anyhow::Result<()> {
    let (stream, _) = listener.accept().await?;
    let writer = Arc::new(Mutex::new(BufWriter::new(stream)));
    //let writer_clone = writer.clone();

    /*let latency_writer = tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(5)).await;
            let mut latencies = Latencies(HashMap::new());
            let info = cluster.info()
            for shard in info {
                let latency = match shard.1.latency().average().map(|d| d.as_millis()) {
                    Some(x) => x as i64,
                    None => continue,
                };
                if latency > 50 {
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
    });*/

    // Event loop
    while let Some((_, event)) = gw_stream.lock().await.next().await {
        match event.unwrap() {
            Message::Text(x) => {
                let mut lock = writer.lock().await;
                ok_or_break!(lock.write_u8(gateway::OP_EVENT).await);
                ok_or_break!(lock.write_u32(x.as_bytes().len() as u32).await);
                ok_or_break!(lock.write_all(x.as_bytes()).await);
                ok_or_break!(lock.flush().await);
            }
            o => {
                println!("{:?}", o);
            }
        }
    }

    //latency_writer.abort();

    Ok(())
}
