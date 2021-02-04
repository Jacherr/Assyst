use bytes::Bytes;
use reqwest::Error;
use serde::Deserialize;

const API_BASE: &str = "https://wsi.jacher.io";

mod routes {
    pub const CAPTION: &str = "/caption";
    pub const REVERSE: &str = "/reverse";
}
#[derive(Deserialize)]
pub struct WsiError {
    pub code: u16,
    pub message: Box<str>
}

pub enum RequestError {
    Reqwest(Error),
    Wsi(WsiError)
}

pub async fn request_bytes(client: &reqwest::Client, route: &str, image: Bytes, query: &[(&str, &str)]) -> Result<Bytes, RequestError> {
    let result = client
        .post(&format!("{}{}", API_BASE, route))
        .header(reqwest::header::AUTHORIZATION, "0192837465")
        .query(query)
        .body(image)
        .send()
        .await
        .map_err(|e| RequestError::Reqwest(e))?;
    return if result.status() != reqwest::StatusCode::OK {
        let json = result.json::<WsiError>().await
            .map_err(|err| RequestError::Reqwest(err))?;
        Err(RequestError::Wsi(json))
    } else {
        let bytes = result.bytes().await
            .map_err(|e| RequestError::Reqwest(e))?;
        Ok(bytes)
    }
}

pub async fn caption(client: &reqwest::Client, image: Bytes, text: &str) -> Result<Bytes, RequestError> {
    request_bytes(client, routes::CAPTION, image, &[("text", text)]).await
}

pub async fn reverse(client: &reqwest::Client, image: Bytes) -> Result<Bytes, RequestError> {
    request_bytes(client, routes::REVERSE, image, &[]).await
}