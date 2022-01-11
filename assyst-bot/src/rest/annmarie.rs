use crate::{assyst::Assyst, util::get_wsi_request_tier};
use bincode::serialize;
use bytes::Bytes;
use futures::Future;
use rand::Rng;
use reqwest::{RequestBuilder, StatusCode};
use serde::{Deserialize, Serialize};
use std::{error::Error as StdError, fmt::Display, pin::Pin, sync::Arc};
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

#[derive(Serialize, Deserialize, Debug)]
pub struct AnnmarieBody {
    pub route: String,
    pub query_params: Vec<(String, String)>,
    pub images: Vec<Vec<u8>>,
}

pub mod routes {
    pub const CARD: &str = "/card";
    pub const DRIP: &str = "/drip";
    pub const FEMURBREAKER: &str = "/femurbreaker";
    pub const FISHEYE: &str = "/fisheye";
    pub const FRINGE: &str = "/fringe";
    pub const F_SHIFT: &str = "/fshift";
    pub const GLOBE: &str = "/globe";
    pub const INFO: &str = "/info";
    pub const LABELS: &str = "/labels";
    pub const MAKESWEET: &str = "/makesweet";
    pub const NEON: &str = "/neon";
    pub const PAINT: &str = "/paint";
    pub const SIREN: &str = "/siren";
    pub const SKETCH: &str = "/sketch";
    pub const SOFTGLOW: &str = "/softglow";
    pub const SWEDEN: &str = "/sweden";
    pub const ZOOM_BLUR: &str = "/zoom-blur";
    pub const QUOTE: &str = "/discord";

    pub const RANDOMIZABLE_ROUTES: &[&str] = &[CARD, FISHEYE, GLOBE, PAINT];

    pub fn command_name_to_route(command: &str) -> Option<&'static str> {
        match command {
            "card" => Some(CARD),
            "fisheye" => Some(FISHEYE),
            "globe" => Some(GLOBE),
            "paint" => Some(PAINT),
            _ => None,
        }
    }
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

#[derive(Deserialize, Debug)]
pub struct AnnmarieLabels {
    pub description: String,
    pub score: f32,
}

#[derive(Debug)]
pub enum RequestError {
    Reqwest(String),
    Annmarie(AnnmarieError, StatusCode),
    InvalidStatus(reqwest::StatusCode),
    Wsi(crate::rest::wsi::RequestError),
}

impl Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestError::Reqwest(x) => write!(f, "{}", &format!("A request error occurred: {}", x)),
            RequestError::Annmarie(_, y) => write!(f, "{}", &format!("Invalid response status code: {:?}", y)),
            RequestError::InvalidStatus(x) => write!(f, "{}", &format!("Invalid response status code: {:?}", x)),
            RequestError::Wsi(x) => write!(f, "{}", &format!("WSI error: {:?}", x))
        }
    }
}

impl StdError for RequestError {}

/// Takes a partially built request, attaches the Authorization header
/// and sends the request, returning any errors
pub async fn finish_request(
    assyst: &Assyst,
    req: RequestBuilder,
    premium_level: usize,
) -> Result<Bytes, RequestError> {
    let result = req
        .header(
            reqwest::header::AUTHORIZATION,
            assyst.config.auth.wsi.as_ref(),
        )
        .header("premium_level", premium_level)
        .send()
        .await
        .map_err(|x| RequestError::Reqwest(format!("{}", x)))?;

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
    assyst: &Assyst,
    route: &str,
    image: Bytes,
    query: &[(&str, &str)],
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let body = AnnmarieBody {
        images: vec![image.to_vec()],
        query_params: query
            .iter()
            .map(|x| (x.0.to_owned(), x.1.to_owned()))
            .collect::<Vec<_>>(),
        route: route.to_owned(),
    };

    let premium_level = get_wsi_request_tier(&assyst.clone(), user_id)
        .await
        .unwrap();

    let req = assyst
        .reqwest_client
        .post(&format!("{}/annmarie", assyst.config.url.wsi))
        .query(query)
        .body(serialize(&body).unwrap());

    finish_request(&assyst, req, premium_level).await
}

pub async fn randomize(
    assyst: &Assyst,
    image: Bytes,
    user_id: UserId,
    acceptable_routes: &mut Vec<&'static str>,
) -> (&'static str, Result<Bytes, RequestError>) {
    let index = rand::thread_rng().gen_range(0..acceptable_routes.len());
    let route = acceptable_routes.remove(index);

    let bytes = request_bytes(assyst, route, image, &[], user_id).await;

    match bytes {
        Ok(bytes) => (route, Ok(bytes)),
        Err(e) => (route, Err(e)),
    }
}

pub async fn quote(
    assyst: &Assyst,
    messages: &[Message],
    guild: Guild,
    white: bool,
) -> Result<Bytes, RequestError> {
    let result = assyst
        .reqwest_client
        .post(&format!("{}{}", assyst.config.url.annmarie, routes::QUOTE))
        .json(&AnnmarieQuote {
            messages,
            guild,
            white,
        })
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

pub async fn billboard(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(
        &assyst,
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
    request_bytes(&assyst, routes::CARD, image, &[], user_id).await
}

pub async fn circuitboard(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(
        &assyst,
        routes::MAKESWEET,
        image,
        &[("template", "circuitboard")],
        user_id,
    )
    .await
}

pub async fn drip(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(&assyst, routes::DRIP, image, &[], user_id).await
}

pub async fn fisheye(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(&assyst, routes::FISHEYE, image, &[], user_id).await
}

pub async fn flag(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(
        &assyst,
        routes::MAKESWEET,
        image,
        &[("template", "flag")],
        user_id,
    )
    .await
}

pub async fn femurbreaker(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(&assyst, routes::FEMURBREAKER, image, &[], user_id).await
}

pub async fn fringe(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(&assyst, routes::FRINGE, image, &[], user_id).await
}

pub async fn f_shift(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(&assyst, routes::F_SHIFT, image, &[], user_id).await
}

pub async fn globe(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(&assyst, routes::GLOBE, image, &[], user_id).await
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

pub async fn labels(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Vec<AnnmarieLabels>, RequestError> {
    let bytes = request_bytes(&assyst, routes::LABELS, image, &[], user_id).await?;
    let string = String::from_utf8_lossy(&bytes).to_string();
    let json = serde_json::from_str::<Vec<AnnmarieLabels>>(&string)
        .map_err(|err| RequestError::Reqwest(format!("{}", err)))?;
    Ok(json)
}

pub async fn neon(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    radius: &str,
) -> Result<Bytes, RequestError> {
    request_bytes(&assyst, routes::NEON, image, &[("radius", radius)], user_id).await
}

pub async fn paint(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(&assyst, routes::PAINT, image, &[], user_id).await
}

pub async fn siren(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(&assyst, routes::SIREN, image, &[], user_id).await
}

pub async fn sketch(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(&assyst, routes::SKETCH, image, &[], user_id).await
}

pub async fn softglow(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(&assyst, routes::SOFTGLOW, image, &[], user_id).await
}

pub async fn sweden(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    request_bytes(&assyst, routes::SWEDEN, image, &[], user_id).await
}

pub async fn zoom_blur(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    power: &str,
) -> Result<Bytes, RequestError> {
    request_bytes(
        &assyst,
        routes::ZOOM_BLUR,
        image,
        &[("power", power)],
        user_id,
    )
    .await
}

pub fn format_err(err: RequestError) -> String {
    match err {
        RequestError::Reqwest(_) => String::from("A network error occurred"),
        RequestError::Annmarie(e, _) => e.message.to_string(),
        RequestError::InvalidStatus(e) => e.to_string(),
        RequestError::Wsi(e) => crate::rest::wsi::format_err(e),
    }
}
