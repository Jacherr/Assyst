use crate::{assyst::Assyst, util::get_wsi_request_tier};
use bincode::serialize;
use bytes::Bytes;
use futures::Future;
use rand::Rng;
use reqwest::{RequestBuilder, StatusCode};
use serde::{Deserialize, Serialize};
use shared::{
    fifo::{FifoData, FifoSend},
    query_params::AnnmarieQueryParams,
};
use std::{error::Error as StdError, fmt::Display, pin::Pin, sync::Arc};
use twilight_model::{channel::Message, guild::Guild, id::UserId};

use super::wsi::run_wsi_job;

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
impl From<crate::rest::wsi::RequestError> for RequestError {
    fn from(e: crate::rest::wsi::RequestError) -> Self {
        RequestError::Wsi(e)
    }
}

impl Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestError::Reqwest(x) => {
                write!(f, "{}", &format!("A request error occurred: {}", x))
            }
            RequestError::Annmarie(_, y) => {
                write!(f, "{}", &format!("Invalid response status code: {:?}", y))
            }
            RequestError::InvalidStatus(x) => {
                write!(f, "{}", &format!("Invalid response status code: {:?}", x))
            }
            RequestError::Wsi(x) => write!(f, "{}", &format!("WSI error: {:?}", x)),
        }
    }
}

impl StdError for RequestError {}

pub async fn randomize(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    acceptable_routes: &mut Vec<&'static str>,
) -> (&'static str, Result<Bytes, RequestError>) {
    let index = rand::thread_rng().gen_range(0..acceptable_routes.len());
    let route = acceptable_routes.remove(index);

    let job = FifoSend::Annmarie(FifoData::new(
        image.to_vec(),
        AnnmarieQueryParams {
            preprocess: true,
            query_params: vec![("template".to_owned(), "flag".to_owned())],
            route: routes::MAKESWEET.to_owned(),
            images: vec![],
        },
    ));

    let x = run_wsi_job(assyst, job, user_id).await;
    (route, {
        if let Ok(bytes) = x {
            Ok(bytes)
        } else if let Err(y) = x{
            Err(RequestError::from(y))
        } else {
            unreachable!()
        }
    })
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
    let job = FifoSend::Annmarie(FifoData::new(
        image.to_vec(),
        AnnmarieQueryParams {
            preprocess: true,
            query_params: vec![("template".to_owned(), "billboard-cityscape".to_owned())],
            route: routes::MAKESWEET.to_owned(),
            images: vec![],
        },
    ));

    Ok(run_wsi_job(assyst, job, user_id).await?)
}

pub async fn card(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Annmarie(FifoData::new(
        image.to_vec(),
        AnnmarieQueryParams {
            preprocess: true,
            query_params: vec![],
            route: routes::CARD.to_owned(),
            images: vec![],
        },
    ));

    Ok(run_wsi_job(assyst, job, user_id).await?)
}

pub async fn circuitboard(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Annmarie(FifoData::new(
        image.to_vec(),
        AnnmarieQueryParams {
            preprocess: true,
            query_params: vec![("template".to_owned(), "circuitboard".to_owned())],
            route: routes::MAKESWEET.to_owned(),
            images: vec![],
        },
    ));

    Ok(run_wsi_job(assyst, job, user_id).await?)
}

pub async fn fisheye(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Annmarie(FifoData::new(
        image.to_vec(),
        AnnmarieQueryParams {
            preprocess: true,
            query_params: vec![],
            route: routes::FISHEYE.to_owned(),
            images: vec![],
        },
    ));

    Ok(run_wsi_job(assyst, job, user_id).await?)
}

pub async fn flag(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Annmarie(FifoData::new(
        image.to_vec(),
        AnnmarieQueryParams {
            preprocess: true,
            query_params: vec![("template".to_owned(), "flag".to_owned())],
            route: routes::MAKESWEET.to_owned(),
            images: vec![],
        },
    ));

    Ok(run_wsi_job(assyst, job, user_id).await?)
}

pub async fn fringe(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Annmarie(FifoData::new(
        image.to_vec(),
        AnnmarieQueryParams {
            preprocess: true,
            query_params: vec![],
            route: routes::FRINGE.to_owned(),
            images: vec![],
        },
    ));

    Ok(run_wsi_job(assyst, job, user_id).await?)
}

pub async fn f_shift(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Annmarie(FifoData::new(
        image.to_vec(),
        AnnmarieQueryParams {
            preprocess: true,
            query_params: vec![],
            route: routes::F_SHIFT.to_owned(),
            images: vec![],
        },
    ));

    Ok(run_wsi_job(assyst, job, user_id).await?)
}

pub async fn globe(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Annmarie(FifoData::new(
        image.to_vec(),
        AnnmarieQueryParams {
            preprocess: true,
            query_params: vec![],
            route: routes::GLOBE.to_owned(),
            images: vec![],
        },
    ));

    Ok(run_wsi_job(assyst, job, user_id).await?)
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
    let job = FifoSend::Annmarie(FifoData::new(
        image.to_vec(),
        AnnmarieQueryParams {
            preprocess: true,
            query_params: vec![],
            route: routes::LABELS.to_owned(),
            images: vec![],
        },
    ));

    let bytes = run_wsi_job(assyst, job, user_id).await?;
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
    let job = FifoSend::Annmarie(FifoData::new(
        image.to_vec(),
        AnnmarieQueryParams {
            preprocess: true,
            query_params: vec![("radius".to_owned(), radius.to_owned())],
            route: routes::NEON.to_owned(),
            images: vec![],
        },
    ));

    Ok(run_wsi_job(assyst, job, user_id).await?)
}

pub async fn paint(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Annmarie(FifoData::new(
        image.to_vec(),
        AnnmarieQueryParams {
            preprocess: true,
            query_params: vec![],
            route: routes::PAINT.to_owned(),
            images: vec![],
        },
    ));

    Ok(run_wsi_job(assyst, job, user_id).await?)
}

pub async fn sketch(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Annmarie(FifoData::new(
        image.to_vec(),
        AnnmarieQueryParams {
            preprocess: true,
            query_params: vec![],
            route: routes::SKETCH.to_owned(),
            images: vec![],
        },
    ));

    Ok(run_wsi_job(assyst, job, user_id).await?)
}

pub async fn softglow(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Annmarie(FifoData::new(
        image.to_vec(),
        AnnmarieQueryParams {
            preprocess: true,
            query_params: vec![],
            route: routes::SOFTGLOW.to_owned(),
            images: vec![],
        },
    ));

    Ok(run_wsi_job(assyst, job, user_id).await?)
}

pub async fn zoom_blur(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    power: &str,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Annmarie(FifoData::new(
        image.to_vec(),
        AnnmarieQueryParams {
            preprocess: true,
            query_params: vec![("power".to_owned(), power.to_owned())],
            route: routes::ZOOM_BLUR.to_owned(),
            images: vec![],
        },
    ));

    Ok(run_wsi_job(assyst, job, user_id).await?)
}

pub fn format_err(err: RequestError) -> String {
    match err {
        RequestError::Reqwest(_) => String::from("A network error occurred"),
        RequestError::Annmarie(e, _) => e.message.to_string(),
        RequestError::InvalidStatus(e) => e.to_string(),
        RequestError::Wsi(e) => crate::rest::wsi::format_err(e),
    }
}
