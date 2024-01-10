use std::{
    collections::HashMap,
    sync::{atomic::AtomicBool, Arc},
    time::Duration,
};

use assyst_common::{
    consts::CACHE_PIPE,
    ok_or_break,
    persistent_cache::{CacheRequest, CacheRequestData, CacheResponse, CacheResponseData, CacheError, CacheResponseInner, guild_cache::TopGuilds},
    some_or_break, unwrap_enum_variant,
};
use bincode::{serialize, deserialize};
use serenity::all::{GuildCreateEvent, GuildDeleteEvent, ReadyEvent};
use tokio::{
    io::{AsyncWriteExt, AsyncReadExt},
    net::UnixStream,
    sync::{mpsc::UnboundedReceiver, oneshot::{Sender, self}, Mutex},
    time::{sleep, timeout},
};
use twilight_model::gateway::payload::incoming::{GuildCreate, GuildDelete, Ready};

use crate::assyst::Assyst;

static CONNECTED: AtomicBool = AtomicBool::new(false);

pub async fn init_guild_caching(
    reciever: UnboundedReceiver<(Sender<CacheResponseInner>, CacheRequestData)>,
) {
    let reciever = Arc::new(Mutex::new(reciever));

    loop {
        let stream = match UnixStream::connect(CACHE_PIPE).await {
            Ok(stream) => stream,
            Err(_) => {
                sleep(Duration::from_secs(10)).await;
                continue;
            }
        };

        CONNECTED.store(true, std::sync::atomic::Ordering::Relaxed);

        let jobs = Arc::new(Mutex::new(HashMap::<usize, Sender<CacheResponseInner>>::new()));
        let jobs_clone = jobs.clone();
        let jobs_clone_2 = jobs.clone();

        let (mut r, mut w) = stream.into_split();

        let mut reader = tokio::spawn(async move {
            loop {
                let len = ok_or_break!(r.read_u32().await);
                let mut buf = vec![0; len as usize];
                ok_or_break!(r.read_exact(&mut buf).await);
                let deserialized = deserialize::<CacheResponse>(&buf).unwrap();
                let id = deserialized.id();
                let tx = jobs_clone.lock().await.remove(&id);
                if let Some(tx) = tx {
                    tx.send(deserialized.data()).expect("failed to send cache job result to sender");
                }
            }
        });

        let reciever = reciever.clone();

        let mut writer = tokio::spawn(async move {
            let mut next_job_id = 0;

            loop {
                let (tx, request) = some_or_break!(reciever.lock().await.recv().await);
                let request = CacheRequest::new(next_job_id, request);
                let job = serialize(&request).unwrap();
                jobs.lock().await.insert(next_job_id, tx);
                next_job_id += 1;
                ok_or_break!(w.write_u32(job.len() as u32).await);
                ok_or_break!(w.write_all(&job).await);
            }
        });

        let _ = tokio::select! {
            output = &mut reader => { writer.abort(); output },
            output = &mut writer => { reader.abort(); output },
        };

        CONNECTED.store(false, std::sync::atomic::Ordering::Relaxed);
        let mut lock = jobs_clone_2.lock().await;
        let keys = lock.keys().map(|x| *x).collect::<Vec<_>>().clone();

        for job in keys {
            let tx = lock.remove(&job).unwrap();
            let _ = tx.send(Err(CacheError::CacheServerDied));
        }

        sleep(Duration::from_secs(10)).await;
    }
}

pub async fn run_cache_job(assyst: Arc<Assyst>, job: CacheRequestData) -> CacheResponseInner {
    if !CONNECTED.load(std::sync::atomic::Ordering::Relaxed) {
        return Err(CacheError::CacheServerDown);
    }

    const TIME_LIMIT: Duration = Duration::from_secs(10);

    let (tx, rx) = oneshot::channel::<CacheResponseInner>();
    assyst.send_to_cache(tx, job);

    let res = timeout(TIME_LIMIT, rx).await;
    match res {
        Err(_) => Err(CacheError::Timeout),
        Ok(x) => x.unwrap_or(Err(CacheError::CacheServerDied))
    }
}

pub async fn get_top_guilds(assyst: Arc<Assyst>) -> anyhow::Result<TopGuilds> {
    let request = CacheRequestData::GetTopGuilds;
    let response = run_cache_job(assyst, request).await?;
    Ok(unwrap_enum_variant!(response, CacheResponseData::TopGuilds))
}

pub async fn get_new_guilds_from_ready(assyst: Arc<Assyst>, event: ReadyEvent) -> anyhow::Result<usize> {
    let request = CacheRequestData::SendReadyEvent(event.into());
    let response = run_cache_job(assyst, request).await?;
    Ok(unwrap_enum_variant!(response, CacheResponseData::TotalNewGuilds))
}

pub async fn handle_guild_create_event(assyst: Arc<Assyst>, event: GuildCreateEvent) -> anyhow::Result<bool> {
    let request = CacheRequestData::SendGuildCreate(event.into());
    let response = run_cache_job(assyst, request).await?;
    Ok(unwrap_enum_variant!(response, CacheResponseData::ShouldLogGuildCreate))
}

pub async fn handle_guild_delete_event(assyst: Arc<Assyst>, event: GuildDeleteEvent) -> anyhow::Result<bool> {
    let request = CacheRequestData::SendGuildDelete(event.into());
    let response = run_cache_job(assyst, request).await?;
    Ok(unwrap_enum_variant!(response, CacheResponseData::ShouldLogGuildDelete))
}

pub async fn get_guild_count(assyst: Arc<Assyst>) -> anyhow::Result<usize> {
    let request = CacheRequestData::GetTotalGuilds;
    let response = run_cache_job(assyst, request).await?;
    Ok(unwrap_enum_variant!(response, CacheResponseData::TotalGuilds))
}