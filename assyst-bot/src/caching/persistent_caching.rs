use std::{sync::{Arc, atomic::AtomicBool}, time::Duration};

use assyst_common::consts::CACHE_PIPE;
use tokio::{sync::{mpsc::UnboundedReceiver, Mutex}, net::{UnixStream}, time::sleep};

static CONNECTED: AtomicBool = AtomicBool::new(false);

pub async fn init_guild_caching(reciever: UnboundedReceiver<()>) {
    let reciever = Arc::new(Mutex::new(reciever));

    loop {
        let stream = match UnixStream::connect(CACHE_PIPE).await {
            Ok(stream) => stream,
            Err(_) => {
                sleep(Duration::from_secs(10)).await;
                continue;
            }
        };

        println!("Connected to guild cache");

        CONNECTED.store(true, std::sync::atomic::Ordering::Relaxed);

        let (mut r, mut w) = stream.into_split();

        let mut reader = tokio::spawn(async move {
            
        });
    }
}