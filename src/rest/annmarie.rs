use crate::assyst::Assyst;
use bytes::Bytes;
use reqwest::StatusCode;
use serde::Deserialize;
use std::sync::Arc;

mod routes {
    pub const GLOBE: &str = "/globe";
    pub const NEON: &str = "/neon";
}

#[derive(Deserialize)]
pub struct AnnmarieError {
    pub message: Box<str>
}

pub enum RequestError {
    Reqwest(String),
    Annmarie(AnnmarieError, StatusCode),
}

pub async fn request_bytes(
    assyst: Arc<Assyst>,
    route: &str,
    image: Bytes,
    query: &[(&str, &str)],
) -> Result<Bytes, RequestError> {
    let result = assyst
        .reqwest_client
        .post(&format!("{}{}", assyst.config.annmarie_url, route))
        .query(query)
        .body(image)
        .send()
        .await
        .map_err(|_| RequestError::Reqwest("A network error occurred".to_owned()))?;
    let status = result.status();
    return if status != reqwest::StatusCode::OK {
        let json = result
            .json::<AnnmarieError>()
            .await
            .map_err(|err| RequestError::Reqwest(err.to_string()))?;
        Err(RequestError::Annmarie(json, status))
    } else {
        let bytes = result.bytes().await.map_err(|e| RequestError::Reqwest(e.to_string()))?;
        Ok(bytes)
    };
}

pub async fn globe(assyst: Arc<Assyst>, image: Bytes) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::GLOBE, image, &[]).await
}

pub async fn neon(assyst: Arc<Assyst>, image: Bytes, radius: &str) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::NEON, image, &[("radius", radius)]).await
}

pub fn format_err(err: RequestError) -> String {
    match err {
        RequestError::Reqwest(e) => e,
        RequestError::Annmarie(e, c) => format!("Error {}: {}", c.as_u16(), e.message),
    }
}
