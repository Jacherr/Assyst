#![feature(never_type)]

use std::{
    rc::Rc,
    sync::Arc,
};

use assyst_common::{
    config::Config,
    consts::{
        gateway::{self},
        EVENT_PIPE,
    },
    ok_or_break,
};
use futures_util::StreamExt;
use tokio::{
    fs::remove_file,
    io::{AsyncWriteExt, BufWriter},
    net::UnixListener,
    sync::Mutex,
};
use twilight_gateway::{
    stream::{create_recommended, ShardMessageStream},
    Config as GatewayConfig, Intents, Message,
};
use twilight_http::Client;
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
    let http_client = Client::new(config.auth.discord.to_string());
    let gateway_config = GatewayConfig::builder(
        config.auth.discord.to_string(),
        Intents::GUILD_MESSAGES | Intents::GUILDS | Intents::MESSAGE_CONTENT,
    )
    .presence(presence)
    .build();

    let mut shards = create_recommended(&http_client, gateway_config.clone(), |_, _| {
        gateway_config.clone()
    })
    .await?
    .collect::<Vec<_>>();
    let stream = Arc::new(Mutex::new(ShardMessageStream::new(shards.iter_mut())));

    let listener = UnixListener::bind(EVENT_PIPE)?;
    let listener = Rc::new(listener);

    loop {
        let _ = supply_connection(listener.clone(), stream.clone()).await;
    }
}

pub async fn supply_connection(
    listener: Rc<UnixListener>,
    gw_stream: Arc<Mutex<ShardMessageStream<'_>>>,
) -> anyhow::Result<()> {
    let (stream, _) = listener.accept().await?;
    let writer = Arc::new(Mutex::new(BufWriter::new(stream)));
    // Event loop
    while let Some((_, event)) = gw_stream.lock().await.next().await {
        match event {
            Ok(Message::Text(x)) => {
                let mut lock = writer.lock().await;
                ok_or_break!(lock.write_u8(gateway::OP_EVENT).await);
                ok_or_break!(lock.write_u32(x.as_bytes().len() as u32).await);
                ok_or_break!(lock.write_all(x.as_bytes()).await);
                ok_or_break!(lock.flush().await);
            }
            o => {
                // Message::Close event or error?
                println!("{:?}", o);
            }
        }
    }

    //latency_writer.abort();

    Ok(())
}
