use bytes::Bytes;
use reqwest::Error;
use serde::Deserialize;
use crate::assyst::Assyst;
use std::sync::Arc;

mod routes {
    pub const CAPTION: &str = "/caption";
    pub const REVERSE: &str = "/reverse";
    pub const SPIN: &str = "/spin";
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

pub async fn request_bytes(assyst: Arc<Assyst>, route: &str, image: Bytes, query: &[(&str, &str)]) -> Result<Bytes, RequestError> {
    let result = assyst.reqwest_client
        .post(&format!("{}{}", assyst.config.wsi_url, route))
        .header(reqwest::header::AUTHORIZATION, assyst.config.wsi_auth.as_ref())
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

pub async fn caption(assyst: Arc<Assyst>, image: Bytes, text: &str) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::CAPTION, image, &[("text", text)]).await
}

pub async fn reverse(assyst: Arc<Assyst>, image: Bytes) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::REVERSE, image, &[]).await
}

pub async fn spin(assyst: Arc<Assyst>, image: Bytes) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::SPIN, image, &[]).await
}

pub fn format_err(err: RequestError) -> String {
    match err {
        RequestError::Reqwest(e) => e.to_string(),
        RequestError::Wsi(e) => format!("Error {}: {}", e.code, e.message)
    }
}