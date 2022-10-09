#![feature(never_type)]

mod state;
mod request_handler;
mod guild_cache;

use std::{rc::Rc, cell::RefCell};

use assyst_common::{consts::CACHE_PIPE, ok_or_break, persistent_cache::{CacheRequest, CacheResponse}};
use bincode::{deserialize, serialize};
use request_handler::handle_request;
use state::State;
use tokio::{net::UnixListener, io::{AsyncReadExt, AsyncWriteExt}, fs::remove_file};

#[tokio::main]
async fn main() -> anyhow::Result<!> {
    let _ = remove_file(CACHE_PIPE).await;
    
    let state = Rc::new(RefCell::new(State::new()));

    let listener = UnixListener::bind(CACHE_PIPE)?;
    loop {
        let (mut stream, _) = listener.accept().await?;

        loop {
            let len = ok_or_break!(stream.read_u32().await);
            let mut data: Vec<u8> = vec![0; len as usize];
            ok_or_break!(stream.read_exact(&mut data).await);
            let request: CacheRequest = deserialize::<CacheRequest>(&data).unwrap();
            let id = request.id();
            let request_data = request.data();
            let response = handle_request(state.clone(), request_data);
            let response = CacheResponse::new(id, response);
            let serialized = serialize(&response).unwrap();
            ok_or_break!(stream.write_u32(serialized.len() as u32).await);
            ok_or_break!(stream.write_all(&serialized).await);
        }
    }
}
