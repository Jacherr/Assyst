use crate::assyst::Assyst;
use crate::util::get_wsi_request_tier;
use bytes::Bytes;
use rand::Rng;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::{future::Future, pin::Pin, sync::Arc};
use twilight_model::id::UserId;

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
    pub const _3D_ROTATE: &str = "/3d_rotate";
    pub const AHSHIT: &str = "/ahshit";
    pub const APRILFOOLS: &str = "/aprilfools";
    pub const AUDIO: &str = "/audio";
    pub const BLUR: &str = "/blur";
    pub const CAPTION: &str = "/caption";
    pub const COMPRESS: &str = "/compress";
    pub const CONVERT_PNG: &str = "/convert_png";
    pub const FIX_TRANSPARENCY: &str = "/fix_transparency";
    pub const FLASH: &str = "/flash";
    pub const FLIP: &str = "/flip";
    pub const FLOP: &str = "/flop";
    pub const FRAMES: &str = "/frames";
    pub const GHOST: &str = "/ghost";
    pub const GIF_LOOP: &str = "/gif_loop";
    pub const GIF_MAGIK: &str = "/gif_magik";
    pub const GIF_SCRAMBLE: &str = "/gif_scramble";
    pub const GIF_SPEED: &str = "/gif_speed";
    pub const GRAYSCALE: &str = "/grayscale";
    pub const HEART_LOCKET: &str = "/heart_locket";
    pub const IMAGE_INFO: &str = "/image_info";
    pub const IMAGEMAGICK_EVAL: &str = "/imagemagick_eval";
    pub const INVERT: &str = "/invert";
    pub const JPEG: &str = "/jpeg";
    pub const MAGIK: &str = "/magik";
    pub const MEME: &str = "/meme";
    pub const MOTIVATE: &str = "/motivate";
    pub const OVERLAY: &str = "/overlay";
    pub const PIXELATE: &str = "/pixelate";
    pub const PREPROCESS: &str = "/preprocess";
    pub const PRINTER: &str = "/printer";
    pub const RAINBOW: &str = "/rainbow";
    pub const RESIZE: &str = "/resize";
    pub const RESTART: &str = "/restart";
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

    pub const RANDOMIZABLE_ROUTES: &[&str] = &[
        FLIP, FLOP, GIF_MAGIK, GRAYSCALE, INVERT, JPEG, MAGIK, RAINBOW, REVERSE, SPIN, SWIRL, TEHI,
    ];
}

#[derive(Deserialize)]
pub struct Stats {
    pub current_requests: usize,
    pub total_workers: usize,
    pub uptime_ms: usize,
}

#[derive(Deserialize, Debug)]
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
    pub comments: Vec<String>,
}

#[derive(Debug)]
pub enum RequestError {
    Reqwest(Error),
    Serde(serde_json::Error),
    Wsi(WsiError),
    Sqlx(sqlx::Error),
}

#[derive(Debug, Serialize)]
pub enum ResizeMethod {
    Nearest,
    Gaussian,
}

impl ResizeMethod {
    pub fn from_str(input: &str) -> Option<Self> {
        match input {
            "nearest" => Some(Self::Nearest),
            "gaussian" => Some(Self::Gaussian),
            _ => None,
        }
    }

    pub fn to_string(&self) -> Result<String, serde_json::Error> {
        let s = serde_json::to_string(self)?;
        Ok(String::from(&s[1..s.len() - 1]))
    }
}

pub async fn randomize(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> (&'static str, Result<Bytes, RequestError>) {
    let index = rand::thread_rng().gen_range(0..routes::RANDOMIZABLE_ROUTES.len());
    let route = routes::RANDOMIZABLE_ROUTES[index];

    let bytes = request_bytes(assyst, route, image, &[], user_id).await;

    match bytes {
        Ok(bytes) => (route, Ok(bytes)),
        Err(e) => (route, Err(e)),
    }
}

pub async fn request_bytes(
    assyst: Arc<Assyst>,
    route: &str,
    image: Bytes,
    query: &[(&str, &str)],
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let premium_level = get_wsi_request_tier(&assyst, user_id)
        .await
        .map_err(RequestError::Sqlx)?;

    let result = assyst
        .reqwest_client
        .post(&format!("{}{}", assyst.config.url.wsi, route))
        .header(
            reqwest::header::AUTHORIZATION,
            assyst.config.auth.wsi.as_ref(),
        )
        .header("premium_level", premium_level)
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

pub async fn _3d_rotate(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::_3D_ROTATE, image, &[], user_id).await
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

pub async fn audio(
    assyst: Arc<Assyst>,
    image: Bytes,
    effect: &str,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::AUDIO, image, &[("effect", effect)], user_id).await
}

pub async fn blur(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    power: &str,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::BLUR, image, &[("power", power)], user_id).await
}

pub async fn caption(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    text: &str,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::CAPTION, image, &[("text", text)], user_id).await
}

pub async fn convert_png(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::CONVERT_PNG, image, &[], user_id).await
}

pub async fn compress(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    level: usize,
) -> Result<Bytes, RequestError> {
    request_bytes(
        assyst,
        routes::COMPRESS,
        image,
        &[("level", &level.to_string())],
        user_id,
    )
    .await
}

pub async fn fix_transparency(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::FIX_TRANSPARENCY, image, &[], user_id).await
}

pub async fn flash(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::FLASH, image, &[], user_id).await
}
pub async fn flip(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::FLIP, image, &[], user_id).await
}

pub async fn flop(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::FLOP, image, &[], user_id).await
}

pub async fn motivate(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    top_text: &str,
    bottom_text: &str,
) -> Result<Bytes, RequestError> {
    request_bytes(
        assyst,
        routes::MOTIVATE,
        image,
        &[("top", top_text), ("bottom", bottom_text)],
        user_id,
    )
    .await
}

pub async fn frames(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::FRAMES, image, &[], user_id).await
}

pub async fn ghost(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    depth: &str,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::GHOST, image, &[("depth", depth)], user_id).await
}

pub async fn gif_loop(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::GIF_LOOP, image, &[], user_id).await
}

pub async fn gif_magik(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::GIF_MAGIK, image, &[], user_id).await
}

pub async fn gif_scramble(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::GIF_SCRAMBLE, image, &[], user_id).await
}

pub async fn grayscale(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::GRAYSCALE, image, &[], user_id).await
}

pub async fn heart_locket(
    assyst: Arc<Assyst>,
    image: Bytes,
    text: &str,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::HEART_LOCKET, image, &[("text", text)], user_id).await
}

pub async fn gif_speed(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    delay: Option<&str>,
) -> Result<Bytes, RequestError> {
    let query = match delay {
        Some(d) => vec![("delay", d)],
        None => vec![],
    };

    request_bytes(assyst, routes::GIF_SPEED, image, &query, user_id).await
}

pub async fn image_info(assyst: Arc<Assyst>, image: Bytes) -> Result<ImageInfo, RequestError> {
    let result = assyst
        .reqwest_client
        .post(&format!("{}{}", assyst.config.url.wsi, routes::IMAGE_INFO))
        .header(
            reqwest::header::AUTHORIZATION,
            assyst.config.auth.wsi.as_ref(),
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
    user_id: UserId,
    script: &str,
) -> Result<Bytes, RequestError> {
    request_bytes(
        assyst,
        routes::IMAGEMAGICK_EVAL,
        image,
        &[("script", script)],
        user_id,
    )
    .await
}

pub async fn invert(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::INVERT, image, &[], user_id).await
}

pub async fn jpeg(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::JPEG, image, &[], user_id).await
}

pub async fn magik(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::MAGIK, image, &[], user_id).await
}

pub async fn meme(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    top: &str,
    bottom: &str,
) -> Result<Bytes, RequestError> {
    request_bytes(
        assyst,
        routes::MEME,
        image,
        &[("top", top), ("bottom", bottom)],
        user_id,
    )
    .await
}

pub async fn pixelate(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    downscaled_height: Option<&str>,
) -> Result<Bytes, RequestError> {
    match downscaled_height {
        Some(d) => {
            request_bytes(
                assyst,
                routes::PIXELATE,
                image,
                &[("downscaled_height", d)],
                user_id,
            )
            .await
        }
        None => request_bytes(assyst, routes::PIXELATE, image, &[], user_id).await,
    }
}

pub async fn preprocess(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::PREPROCESS, image, &[], user_id).await
}

pub async fn printer(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::PRINTER, image, &[], user_id).await
}

pub async fn rainbow(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::RAINBOW, image, &[], user_id).await
}

pub async fn resize(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    method: ResizeMethod,
) -> Result<Bytes, RequestError> {
    let method = method.to_string().map_err(RequestError::Serde)?;

    request_bytes(
        assyst,
        routes::RESIZE,
        image,
        &[("method", &method)],
        user_id,
    )
    .await
}

pub async fn resize_scale(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    scale: f32,
    method: ResizeMethod,
) -> Result<Bytes, RequestError> {
    let method = method.to_string().map_err(RequestError::Serde)?;

    request_bytes(
        assyst,
        routes::RESIZE,
        image,
        &[("scale", &scale.to_string()), ("method", &method)],
        user_id,
    )
    .await
}

pub async fn resize_width_height(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    width: usize,
    height: usize,
    method: ResizeMethod,
) -> Result<Bytes, RequestError> {
    let method = method.to_string().map_err(RequestError::Serde)?;

    request_bytes(
        assyst,
        routes::RESIZE,
        image,
        &[
            ("width", &width.to_string()),
            ("height", &height.to_string()),
            ("method", &method),
        ],
        user_id,
    )
    .await
}

pub async fn overlay(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    overlay: &str,
) -> Result<Bytes, RequestError> {
    request_bytes(
        assyst,
        routes::OVERLAY,
        image,
        &[("overlay", overlay)],
        user_id,
    )
    .await
}

pub async fn restart(assyst: Arc<Assyst>) -> Result<(), RequestError> {
    let result = assyst
        .reqwest_client
        .get(&format!("{}{}", assyst.config.url.wsi, routes::RESTART))
        .header(
            reqwest::header::AUTHORIZATION,
            assyst.config.auth.wsi.as_ref(),
        )
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
        Ok(())
    };
}

pub async fn reverse(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::REVERSE, image, &[], user_id).await
}

pub async fn rotate(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    degrees: &str,
) -> Result<Bytes, RequestError> {
    request_bytes(
        assyst,
        routes::ROTATE,
        image,
        &[("degrees", degrees)],
        user_id,
    )
    .await
}

pub async fn set_loop(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    looping: bool,
) -> Result<Bytes, RequestError> {
    request_bytes(
        assyst,
        routes::SET_LOOP,
        image,
        &[("loop", &looping.to_string())],
        user_id,
    )
    .await
}

pub async fn spin(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::SPIN, image, &[], user_id).await
}

pub async fn spread(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::SPREAD, image, &[], user_id).await
}

pub async fn stats(assyst: Arc<Assyst>) -> Result<Stats, RequestError> {
    let result = assyst
        .reqwest_client
        .get(&format!("{}{}", assyst.config.url.wsi, routes::STATS))
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

pub async fn swirl(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::SWIRL, image, &[], user_id).await
}

pub async fn tehi(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::TEHI, image, &[], user_id).await
}

pub async fn wall(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::WALL, image, &[], user_id).await
}

pub async fn wave(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::WAVE, image, &[], user_id).await
}

pub async fn wormhole(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::WORMHOLE, image, &[], user_id).await
}

pub async fn zoom(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(assyst, routes::ZOOM, image, &[], user_id).await
}

pub fn format_err(err: RequestError) -> String {
    match err {
        RequestError::Reqwest(_) => String::from("A network error occurred"),
        RequestError::Wsi(e) => e.message.to_string(),
        RequestError::Serde(e) => e.to_string(),
        RequestError::Sqlx(e) => e.to_string(),
    }
}
