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
    pub const FLASH: &str = "/flash";
    pub const FLIP: &str = "/flip";
    pub const FLOP: &str = "/flop";
    pub const GHOST: &str = "/ghost";
    pub const GIF_LOOP: &str = "/gif_loop";
    pub const GIF_MAGIK: &str = "/gif_magik";
    pub const GIF_SCRAMBLE: &str = "/gif_scramble";
    pub const GIF_SPEED: &str = "/gif_speed";
    pub const GRAYSCALE: &str = "/grayscale";
    pub const IMAGE_INFO: &str = "/image_info";
    pub const IMAGEMAGICK_EVAL: &str = "/imagemagick_eval";
    pub const INVERT: &str = "/invert";
    pub const JPEG: &str = "/jpeg";
    pub const MAGIK: &str = "/magik";
    pub const MEME: &str = "/meme";
    pub const MOTIVATE: &str = "/motivate";
    pub const OVERLAY: &str = "/overlay";
    pub const PREPROCESS: &str = "/preprocess";
    pub const PRINTER: &str = "/printer";
    pub const RAINBOW: &str = "/rainbow";
    pub const RESIZE: &str = "/resize";
    pub const REVERSE: &str = "/reverse";
    pub const ROTATE: &str = "/rotate";
    pub const SET_LOOP: &str = "/set_loop";
    pub const SPIN: &str = "/spin";
    pub const SPREAD: &str = "/spread";
    pub const STATS: &str = "/stats";
    pub const SWIRL: &str = "/swirl";
    pub const TEHI: &str = "/tehi";
    pub const WALL: &str = "/wall";
    pub const WAVE: &str = "/wave";
    pub const WORMHOLE: &str = "/wormhole";
    pub const ZOOM: &str = "/zoom";
}

#[derive(Deserialize)]
pub struct Stats {
    pub current_requests: usize,
    pub total_workers: usize,
}

#[derive(Deserialize)]
pub struct WsiError {
    pub code: u16,
    pub message: Box<str>,
}
#[derive(Deserialize)]
pub struct ImageInfo {
    pub file_size_bytes: usize,
    pub mime_type: String,
    pub dimensions: (u32, u32),
    pub colour_space: String,
    pub frames: Option<usize>,
    pub frame_delays: Option<Vec<usize>>,
    pub repeat: Option<isize>,
    pub comments: Vec<String>
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
        .header("premium_level", 0)
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

pub async fn flash(assyst: Arc<Assyst>, image: Bytes) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::FLASH, image, &[]).await
}
pub async fn flip(assyst: Arc<Assyst>, image: Bytes) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::FLIP, image, &[]).await
}

pub async fn flop(assyst: Arc<Assyst>, image: Bytes) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::FLOP, image, &[]).await
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

pub async fn ghost(assyst: Arc<Assyst>, image: Bytes, depth: &str) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::GHOST, image, &[("depth", depth)]).await
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
    delay: Option<&str>,
) -> Result<Bytes, RequestError> {
    let query = match delay {
        Some(d) => vec![("delay", d)],
        None => vec![],
    };

    request_bytes(assyst, routes::GIF_SPEED, image, &query).await
}

pub async fn image_info(assyst: Arc<Assyst>, image: Bytes) -> Result<ImageInfo, RequestError> {
    let result = assyst
        .reqwest_client
        .post(&format!("{}{}", assyst.config.wsi_url, routes::IMAGE_INFO))
        .header(
            reqwest::header::AUTHORIZATION,
            assyst.config.wsi_auth.as_ref(),
        )
        .header("premium_level", 0)
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
        let json = result
            .json::<ImageInfo>()
            .await
            .map_err(|err| RequestError::Reqwest(err))?;
        Ok(json)
    };
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

pub async fn jpeg(assyst: Arc<Assyst>, image: Bytes) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::JPEG, image, &[]).await
}

pub async fn magik(assyst: Arc<Assyst>, image: Bytes) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::MAGIK, image, &[]).await
}

pub async fn meme(
    assyst: Arc<Assyst>,
    image: Bytes,
    top: &str,
    bottom: &str,
) -> Result<Bytes, RequestError> {
    request_bytes(
        assyst,
        routes::MEME,
        image,
        &[("top", top), ("bottom", bottom)],
    )
    .await
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

pub async fn resize(assyst: Arc<Assyst>, image: Bytes) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::RESIZE, image, &[]).await
}

pub async fn resize_scale(
    assyst: Arc<Assyst>,
    image: Bytes,
    scale: f32,
) -> Result<Bytes, RequestError> {
    request_bytes(
        assyst,
        routes::RESIZE,
        image,
        &[("scale", &scale.to_string())],
    )
    .await
}

pub async fn resize_width_height(
    assyst: Arc<Assyst>,
    image: Bytes,
    width: usize,
    height: usize,
) -> Result<Bytes, RequestError> {
    request_bytes(
        assyst,
        routes::RESIZE,
        image,
        &[
            ("width", &width.to_string()),
            ("height", &height.to_string()),
        ],
    )
    .await
}

pub async fn overlay(
    assyst: Arc<Assyst>,
    image: Bytes,
    overlay: &str,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::OVERLAY, image, &[("overlay", overlay)]).await
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

pub async fn stats(assyst: Arc<Assyst>) -> Result<Stats, RequestError> {
    let result = assyst
        .reqwest_client
        .get(&format!("{}{}", assyst.config.wsi_url, routes::STATS))
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
        let json = result
            .json::<Stats>()
            .await
            .map_err(|err| RequestError::Reqwest(err))?;
        Ok(json)
    };
}

pub async fn swirl(assyst: Arc<Assyst>, image: Bytes) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::SWIRL, image, &[]).await
}

pub async fn tehi(assyst: Arc<Assyst>, image: Bytes) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::TEHI, image, &[]).await
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
        RequestError::Wsi(e) => e.message.to_string(),
    }
}
