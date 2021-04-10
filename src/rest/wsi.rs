use crate::assyst::Assyst;
use bytes::Bytes;
use reqwest::Error;
use serde::Deserialize;
use std::{future::Future, pin::Pin, sync::Arc};

pub type NoArgFunction = Box<
    dyn Fn(Arc<Assyst>, Bytes) -> Pin<Box<dyn Future<Output = Result<Bytes, RequestError>> + Send>>
        + Send
        + Sync,
>;

mod routes {
    pub const _3D_ROTATE: &str = "/3d_rotate";
    pub const CAPTION: &str = "/caption";
    pub const COMPRESS: &str = "/compress";
    pub const FIX_TRANSPARENCY: &str = "/fix_transparency";
    pub const GIF_LOOP: &str = "/gif_loop";
    pub const GIF_MAGIK: &str = "/gif_magik";
    pub const GIF_SCRAMBLE: &str = "/gif_scramble";
    pub const GIF_SPEED: &str = "/gif_speed";
    pub const GRAYSCALE: &str = "/grayscale";
    pub const IMAGEMAGICK_EVAL: &str = "/imagemagick_eval";
    pub const INVERT: &str = "/invert";
    pub const MAGIK: &str = "/magik";
    pub const MOTIVATE: &str = "/motivate";
    pub const PREPROCESS: &str = "/preprocess";
    pub const PRINTER: &str = "/printer";
    pub const RAINBOW: &str = "/rainbow";
    pub const REVERSE: &str = "/reverse";
    pub const ROTATE: &str = "/rotate";
    pub const SET_LOOP: &str = "/set_loop";
    pub const SPIN: &str = "/spin";
    pub const SPREAD: &str = "/spread";
    pub const SWIRL: &str = "/swirl";
    pub const WALL: &str = "/wall";
    pub const WAVE: &str = "/wave";
    pub const WORMHOLE: &str = "/wormhole";
    pub const ZOOM: &str = "/zoom";
}
#[derive(Deserialize)]
pub struct WsiError {
    pub code: u16,
    pub message: Box<str>,
}

pub enum RequestError {
    Reqwest(Error),
    Wsi(WsiError),
}

pub async fn request_bytes(
    assyst: Arc<Assyst>,
    route: &str,
    image: Bytes,
    query: &[(&str, &str)],
) -> Result<Bytes, RequestError> {
    let result = assyst
        .reqwest_client
        .post(&format!("{}{}", assyst.config.wsi_url, route))
        .header(
            reqwest::header::AUTHORIZATION,
            assyst.config.wsi_auth.as_ref(),
        )
        .query(query)
        .body(image)
        .send()
        .await
        .map_err(|e| RequestError::Reqwest(e))?;
    return if result.status() != reqwest::StatusCode::OK {
        let json = result
            .json::<WsiError>()
            .await
            .map_err(|err| RequestError::Reqwest(err))?;
        Err(RequestError::Wsi(json))
    } else {
        let bytes = result.bytes().await.map_err(|e| RequestError::Reqwest(e))?;
        Ok(bytes)
    };
}

pub async fn _3d_rotate(assyst: Arc<Assyst>, image: Bytes) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::_3D_ROTATE, image, &[]).await
}

pub async fn caption(assyst: Arc<Assyst>, image: Bytes, text: &str) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::CAPTION, image, &[("text", text)]).await
}

pub async fn compress(
    assyst: Arc<Assyst>,
    image: Bytes,
    level: usize,
) -> Result<Bytes, RequestError> {
    request_bytes(
        assyst,
        routes::COMPRESS,
        image,
        &[("level", &level.to_string())],
    )
    .await
}

pub async fn fix_transparency(assyst: Arc<Assyst>, image: Bytes) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::FIX_TRANSPARENCY, image, &[]).await
}

pub async fn motivate(
    assyst: Arc<Assyst>,
    image: Bytes,
    top_text: &str,
    bottom_text: &str,
) -> Result<Bytes, RequestError> {
    request_bytes(
        assyst,
        routes::MOTIVATE,
        image,
        &[("top", top_text), ("bottom", bottom_text)],
    )
    .await
}

pub async fn gif_loop(assyst: Arc<Assyst>, image: Bytes) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::GIF_LOOP, image, &[]).await
}

pub async fn gif_magik(assyst: Arc<Assyst>, image: Bytes) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::GIF_MAGIK, image, &[]).await
}

pub async fn gif_scramble(assyst: Arc<Assyst>, image: Bytes) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::GIF_SCRAMBLE, image, &[]).await
}

pub async fn grayscale(assyst: Arc<Assyst>, image: Bytes) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::GRAYSCALE, image, &[]).await
}

pub async fn gif_speed(
    assyst: Arc<Assyst>,
    image: Bytes,
    delay: &str,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::GIF_SPEED, image, &[("delay", delay)]).await
}

pub async fn imagemagick_eval(
    assyst: Arc<Assyst>,
    image: Bytes,
    script: &str,
) -> Result<Bytes, RequestError> {
    request_bytes(
        assyst,
        routes::IMAGEMAGICK_EVAL,
        image,
        &[("script", script)],
    )
    .await
}

pub async fn invert(assyst: Arc<Assyst>, image: Bytes) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::INVERT, image, &[]).await
}

pub async fn magik(assyst: Arc<Assyst>, image: Bytes) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::MAGIK, image, &[]).await
}

pub async fn preprocess(assyst: Arc<Assyst>, image: Bytes) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::PREPROCESS, image, &[]).await
}

pub async fn printer(assyst: Arc<Assyst>, image: Bytes) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::PRINTER, image, &[]).await
}

pub async fn rainbow(assyst: Arc<Assyst>, image: Bytes) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::RAINBOW, image, &[]).await
}

pub async fn reverse(assyst: Arc<Assyst>, image: Bytes) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::REVERSE, image, &[]).await
}

pub async fn rotate(
    assyst: Arc<Assyst>,
    image: Bytes,
    degrees: &str,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::ROTATE, image, &[("degrees", degrees)]).await
}

pub async fn set_loop(
    assyst: Arc<Assyst>,
    image: Bytes,
    looping: bool,
) -> Result<Bytes, RequestError> {
    request_bytes(
        assyst,
        routes::SET_LOOP,
        image,
        &[("loop", &looping.to_string())],
    )
    .await
}

pub async fn spin(assyst: Arc<Assyst>, image: Bytes) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::SPIN, image, &[]).await
}

pub async fn spread(assyst: Arc<Assyst>, image: Bytes) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::SPREAD, image, &[]).await
}

pub async fn swirl(assyst: Arc<Assyst>, image: Bytes) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::SWIRL, image, &[]).await
}

pub async fn wall(assyst: Arc<Assyst>, image: Bytes) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::WALL, image, &[]).await
}

pub async fn wave(assyst: Arc<Assyst>, image: Bytes) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::WAVE, image, &[]).await
}

pub async fn wormhole(assyst: Arc<Assyst>, image: Bytes) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::WORMHOLE, image, &[]).await
}

pub async fn zoom(assyst: Arc<Assyst>, image: Bytes) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::ZOOM, image, &[]).await
}

pub fn format_err(err: RequestError) -> String {
    match err {
        RequestError::Reqwest(e) => e.to_string(),
        RequestError::Wsi(e) => e.message.to_string()
    }
}
