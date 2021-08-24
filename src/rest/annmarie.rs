use crate::{assyst::Assyst, rest::wsi::preprocess};
use bytes::Bytes;
use futures::Future;
use rand::Rng;
use reqwest::{RequestBuilder, StatusCode};
use serde::{Deserialize, Serialize};
use std::{pin::Pin, sync::Arc};
use twilight_model::{channel::Message, guild::Guild, id::UserId};

pub type NoArgFunction = Box<
    dyn Fn(
            Arc<Assyst>,
            Bytes,
            UserId,
        ) -> Pin<Box<dyn Future<Output = Result<Bytes, RequestError>> + Send>>
        + Send
        + Sync,
>;

pub mod routes {
    pub const AHSHIT: &str = "/ahshit";
    pub const APRILFOOLS: &str = "/april-fools";
    pub const CARD: &str = "/card";
    pub const FISHEYE: &str = "/fisheye";
    pub const FRINGE: &str = "/fringe";
    pub const F_SHIFT: &str = "/fshift";
    pub const GLOBE: &str = "/globe";
    pub const INFO: &str = "/info";
    pub const MAKESWEET: &str = "/makesweet";
    pub const NEON: &str = "/neon";
    pub const PAINT: &str = "/paint";
    pub const SKETCH: &str = "/sketch";
    pub const TERRARIA: &str = "/terraria";
    pub const ZOOM_BLUR: &str = "/zoom-blur";
    pub const QUOTE: &str = "/discord";

    pub const RANDOMIZABLE_ROUTES: &[&str] = &[CARD, FISHEYE, F_SHIFT, GLOBE, PAINT];
}

#[derive(Deserialize, Debug)]
pub struct AnnmarieError {
    pub message: Box<str>,
}

#[derive(Deserialize)]
pub struct AnnmarieInfo {
    pub uptime: f64,
}

#[derive(Serialize, Debug)]
pub struct AnnmarieQuote<'a> {
    messages: &'a [Message],
    guild: Guild,
    white: bool,
}

#[derive(Debug)]
pub enum RequestError {
    Reqwest(String),
    Annmarie(AnnmarieError, StatusCode),
    InvalidStatus(reqwest::StatusCode),
    Wsi(crate::rest::wsi::RequestError),
}

/// Takes a partially built request, attaches the Authorization header
/// and sends the request, returning any errors
pub async fn finish_request(assyst: &Assyst, req: RequestBuilder) -> Result<Bytes, RequestError> {
    let result = req
        .header(
            reqwest::header::AUTHORIZATION,
            assyst.config.auth.annmarie.as_ref(),
        )
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

pub async fn request_bytes(
    assyst: Arc<Assyst>,
    route: &str,
    image: Bytes,
    query: &[(&str, &str)],
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let new_image = preprocess(assyst.clone(), image, user_id)
        .await
        .map_err(|e| RequestError::Wsi(e))?;

    let req = assyst
        .reqwest_client
        .post(&format!("{}{}", assyst.config.url.annmarie, route))
        .query(query)
        .body(new_image);

    finish_request(&assyst, req).await
}

pub async fn randomize(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<(&'static str, Bytes), RequestError> {
    let index = rand::thread_rng().gen_range(0..routes::RANDOMIZABLE_ROUTES.len());
    let route = routes::RANDOMIZABLE_ROUTES[index];

    let bytes = request_bytes(assyst, route, image, &[], user_id).await?;

    Ok((route, bytes))
}

pub async fn quote(
    assyst: &Assyst,
    messages: &[Message],
    guild: Guild,
    white: bool,
) -> Result<Bytes, RequestError> {
    let req = assyst
        .reqwest_client
        .post(&format!("{}{}", assyst.config.url.annmarie, routes::QUOTE))
        .json(&AnnmarieQuote {
            messages,
            guild,
            white,
        });

    finish_request(&assyst, req).await
}

pub async fn ahshit(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::AHSHIT, image, &[], user_id).await
}

pub async fn aprilfools(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::APRILFOOLS, image, &[], user_id).await
}

pub async fn billboard(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(
        assyst,
        routes::MAKESWEET,
        image,
        &[("template", "billboard-cityscape")],
        user_id,
    )
    .await
}

pub async fn card(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::CARD, image, &[], user_id).await
}

pub async fn circuitboard(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(
        assyst,
        routes::MAKESWEET,
        image,
        &[("template", "circuitboard")],
        user_id,
    )
    .await
}

pub async fn fisheye(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::FISHEYE, image, &[], user_id).await
}

pub async fn flag(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(
        assyst,
        routes::MAKESWEET,
        image,
        &[("template", "flag")],
        user_id,
    )
    .await
}

pub async fn fringe(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::FRINGE, image, &[], user_id).await
}

pub async fn f_shift(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::F_SHIFT, image, &[], user_id).await
}

pub async fn globe(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::GLOBE, image, &[], user_id).await
}

pub async fn info(assyst: Arc<Assyst>) -> Result<AnnmarieInfo, RequestError> {
    let result = assyst
        .reqwest_client
        .get(&format!("{}{}", assyst.config.url.annmarie, routes::INFO))
        .send()
        .await
        .map_err(|e| RequestError::Reqwest(e.to_string()))?;

    let status = result.status();

    return if status != reqwest::StatusCode::OK {
        let json = result
            .json::<AnnmarieError>()
            .await
            .map_err(|err| RequestError::Reqwest(err.to_string()))?;
        Err(RequestError::Annmarie(json, status))
    } else {
        let json = result
            .json::<AnnmarieInfo>()
            .await
            .map_err(|err| RequestError::Reqwest(err.to_string()))?;
        Ok(json)
    };
}

pub async fn neon(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    radius: &str,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::NEON, image, &[("radius", radius)], user_id).await
}

pub async fn paint(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::PAINT, image, &[], user_id).await
}

pub async fn sketch(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::SKETCH, image, &[], user_id).await
}

pub async fn terraria(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::TERRARIA, image, &[], user_id).await
}

pub async fn zoom_blur(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    power: &str,
) -> Result<Bytes, RequestError> {
    request_bytes(
        assyst,
        routes::ZOOM_BLUR,
        image,
        &[("power", power)],
        user_id,
    )
    .await
}

pub fn format_err(err: RequestError) -> String {
    match err {
        RequestError::Reqwest(e) => e,
        RequestError::Annmarie(e, _) => e.message.to_string(),
        RequestError::InvalidStatus(e) => e.to_string(),
        RequestError::Wsi(e) => crate::rest::wsi::format_err(e),
    }
}
