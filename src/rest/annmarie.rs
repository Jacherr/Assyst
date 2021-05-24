use crate::{assyst::Assyst, rest::wsi::preprocess};
use bytes::Bytes;
use futures::Future;
use reqwest::StatusCode;
use serde::Deserialize;
use std::{pin::Pin, sync::Arc};

pub type NoArgFunction = Box<
    dyn Fn(Arc<Assyst>, Bytes) -> Pin<Box<dyn Future<Output = Result<Bytes, RequestError>> + Send>>
        + Send
        + Sync,
>;

mod routes {
    pub const AHSHIT: &str = "/ahshit";
    pub const APRILFOOLS: &str = "/april-fools";
    pub const CARD: &str = "/card";
    pub const FISHEYE: &str = "/fisheye";
    pub const FRINGE: &str = "/fringe";
    pub const F_SHIFT: &str = "/fshift";
    pub const GLOBE: &str = "/globe";
    pub const MAKESWEET: &str = "/makesweet";
    pub const NEON: &str = "/neon";
    pub const PAINT: &str = "/paint";
    pub const SKETCH: &str = "/sketch";
    pub const TERRARIA: &str = "/terraria";
    pub const ZOOM_BLUR: &str = "/zoom-blur";
}

#[derive(Deserialize)]
pub struct AnnmarieError {
    pub message: Box<str>,
}

pub enum RequestError {
    Reqwest(String),
    Annmarie(AnnmarieError, StatusCode),
    InvalidStatus(reqwest::StatusCode),
    Wsi(crate::rest::wsi::RequestError),
}

pub async fn request_bytes(
    assyst: Arc<Assyst>,
    route: &str,
    image: Bytes,
    query: &[(&str, &str)],
) -> Result<Bytes, RequestError> {
    let new_image = preprocess(assyst.clone(), image)
        .await
        .map_err(|e| RequestError::Wsi(e))?;

    let result = assyst
        .reqwest_client
        .post(&format!("{}{}", assyst.config.annmarie_url, route))
        .header(
            reqwest::header::AUTHORIZATION,
            assyst.config.annmarie_auth.as_ref(),
        )
        .query(query)
        .body(new_image)
        .send()
        .await
        .map_err(|_| RequestError::Reqwest("A network error occurred".to_owned()))?;

    let status = result.status();

    return if status != reqwest::StatusCode::OK {
        let json = result.json::<AnnmarieError>().await.map_err(|_| {
            RequestError::Reqwest("There was an error decoding the response".to_owned())
        })?;
        Err(RequestError::Annmarie(json, status))
    } else {
        let bytes = result
            .bytes()
            .await
            .map_err(|e| RequestError::Reqwest(e.to_string()))?;
        Ok(bytes)
    };
}

pub async fn ahshit(assyst: Arc<Assyst>, image: Bytes) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::AHSHIT, image, &[]).await
}

pub async fn aprilfools(assyst: Arc<Assyst>, image: Bytes) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::APRILFOOLS, image, &[]).await
}

pub async fn billboard(assyst: Arc<Assyst>, image: Bytes) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::MAKESWEET, image, &[("template", "billboard-cityscape")]).await
}

pub async fn card(assyst: Arc<Assyst>, image: Bytes) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::CARD, image, &[]).await
}

pub async fn fisheye(assyst: Arc<Assyst>, image: Bytes) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::FISHEYE, image, &[]).await
}

pub async fn flag(assyst: Arc<Assyst>, image: Bytes) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::MAKESWEET, image, &[("template", "flag")]).await
}

pub async fn fringe(assyst: Arc<Assyst>, image: Bytes) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::FRINGE, image, &[]).await
}

pub async fn f_shift(assyst: Arc<Assyst>, image: Bytes) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::F_SHIFT, image, &[]).await
}

pub async fn globe(assyst: Arc<Assyst>, image: Bytes) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::GLOBE, image, &[]).await
}

pub async fn neon(assyst: Arc<Assyst>, image: Bytes, radius: &str) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::NEON, image, &[("radius", radius)]).await
}

pub async fn paint(assyst: Arc<Assyst>, image: Bytes) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::PAINT, image, &[]).await
}

pub async fn sketch(assyst: Arc<Assyst>, image: Bytes) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::SKETCH, image, &[]).await
}

pub async fn terraria(assyst: Arc<Assyst>, image: Bytes) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::TERRARIA, image, &[]).await
}

pub async fn zoom_blur(
    assyst: Arc<Assyst>,
    image: Bytes,
    power: &str,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::ZOOM_BLUR, image, &[("power", power)]).await
}

pub fn format_err(err: RequestError) -> String {
    match err {
        RequestError::Reqwest(e) => e,
        RequestError::Annmarie(e, _) => e.message.to_string(),
        RequestError::InvalidStatus(e) => e.to_string(),
        RequestError::Wsi(e) => crate::rest::wsi::format_err(e),
    }
}
