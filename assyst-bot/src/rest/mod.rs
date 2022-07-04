use std::{fmt::Display, sync::Arc};

use assyst_common::{
    eval::{FakeEvalBody, FakeEvalImageResponse},
    filetype, util::UserId,
};
use bytes::Bytes;
use reqwest::{Client, ClientBuilder, Error};
use serde::Deserialize;
use serde_json::json;
use shared::{
    fifo::{FifoData, FifoSend},
    query_params::NoneQuery,
};
use tokio::time::Instant;

use std::error::Error as StdError;

use crate::{
    ansi::Ansi,
    assyst::Assyst,
    downloader,
    rest::wsi::run_wsi_job,
    util,
};

use self::rust::OptimizationLevel;

pub mod bt;
pub mod codesprint;
pub mod identify;
pub mod patreon;
pub mod rust;
pub mod wombo;
pub mod wsi;

mod routes {
    use assyst_common::consts::BOT_ID;

    pub const COOL_TEXT: &str = "https://cooltext.com/PostChange";
    pub const OCR: &str = "http://ocr.y21_.repl.co/?url=";
    pub const CHARINFO: &str = "https://www.fileformat.info/info/unicode/char/";
    pub const IDENTIFY: &str = "https://microsoft-computer-vision3.p.rapidapi.com/analyze?language=en&descriptionExclude=Celebrities&visualFeatures=Description&details=Celebrities";

    pub fn discord_bot_list_stats_url() -> String {
        format!("https://discordbotlist.com/api/v1/bots/{}/stats", BOT_ID)
    }

    pub fn top_gg_stats_url() -> String {
        format!("https://top.gg/api/bots/{}/stats", BOT_ID)
    }

    pub fn discords_stats_url() -> String {
        format!("https://discords.com/bots/api/bot/{}", BOT_ID)
    }
}

#[derive(Debug, Clone)]
pub enum OcrError {
    NetworkError,
    HtmlResponse,
}

impl Display for OcrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OcrError::NetworkError => write!(f, "An unknown network error occurred"),
            OcrError::HtmlResponse => write!(f, "Failed to parse response"),
        }
    }
}

impl StdError for OcrError {}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoolTextResponse {
    pub logo_id: usize,
    pub new_id: usize,
    pub render_location: String,
    pub is_animated: bool,
}

#[derive(Deserialize)]
pub struct Rule34Result {
    pub url: String,
    pub score: u32,
}

pub async fn ocr_image(client: &Client, url: &str) -> Result<String, OcrError> {
    let text = client
        .get(format!("{}{}", routes::OCR, url))
        .send()
        .await
        .map_err(|_| OcrError::NetworkError)?
        .error_for_status()
        .map_err(|_| OcrError::NetworkError)?
        .text()
        .await
        .map_err(|_| OcrError::NetworkError)?;

    if util::starts_with_case_insensitive(text.as_bytes(), b"<!doctype html>") {
        return Err(OcrError::HtmlResponse);
    }

    return Ok(text);
}

pub async fn upload_to_filer(
    assyst: Arc<Assyst>,
    data: Bytes,
    content_type: &str,
) -> Result<String, Error> {
    assyst
        .reqwest_client
        .post(assyst.config.url.cdn.to_string())
        .header(reqwest::header::CONTENT_TYPE, content_type)
        .header(
            reqwest::header::AUTHORIZATION,
            assyst.config.auth.cdn.to_string(),
        )
        .body(data)
        .send()
        .await?
        .text()
        .await
}

pub async fn get_char_info(client: &Client, ch: char) -> Result<(String, String), Error> {
    let url = format!("{}{:x}", routes::CHARINFO, ch as u32);

    Ok((client.get(&url).send().await?.text().await?, url))
}

pub fn parse_path_parameter(path: String, param: (&str, &str)) -> String {
    path.replace(&format!(":{}", param.0), param.1)
}

pub async fn fake_eval(
    assyst: &Assyst,
    code: &str,
    image: bool,
) -> anyhow::Result<FakeEvalImageResponse> {
    let result = assyst
        .reqwest_client
        .post(format!("{}/eval", assyst.config.url.eval))
        .query(&[("returnBuffer", &image.to_string())])
        .json(&FakeEvalBody {
            code: code.to_string(),
        })
        .send()
        .await?
        .bytes()
        .await?;

    if let Some(sig) = filetype::get_sig(&result) {
        Ok(FakeEvalImageResponse::Image(result, sig))
    } else {
        let text = std::str::from_utf8(&result)?;

        serde_json::from_str(text)
            .map(FakeEvalImageResponse::Text)
            .map_err(Into::into)
    }
}

pub async fn burning_text(text: &str) -> Result<Bytes, Error> {
    let client = ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let cool_text_response = client
        .post(routes::COOL_TEXT)
        .query(&[
            ("LogoID", "4"),
            ("Text", text),
            ("FontSize", "70"),
            ("Color1_color", "#FF0000"),
            ("Integer1", "15"),
            ("Boolean1", "on"),
            ("Integer9", "0"),
            ("Integer13", "on"),
            ("Integer12", "on"),
            ("BackgroundColor_color", "#FFFFFF"),
        ])
        .header("content-length", "0")
        .send()
        .await?
        .json::<CoolTextResponse>()
        .await?;

    let url = cool_text_response.render_location;
    let content = client
        .get(&url.replace("https", "http"))
        .send()
        .await?
        .bytes()
        .await?;

    Ok(content)
}

pub async fn post_bot_stats(
    client: &Client,
    top_gg_token: &str,
    guild_count: u32,
) -> Result<(), Error> {
    client
        .post(routes::top_gg_stats_url())
        .header("authorization", top_gg_token)
        .json(&json!({ "server_count": guild_count }))
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}

pub async fn convert_lottie_to_gif(assyst: &Assyst, lottie: &str) -> Result<Bytes, Error> {
    Ok(assyst
        .reqwest_client
        .post(&assyst.config.url.lottie_render.to_string())
        .header(
            "authorization",
            &assyst.config.auth.lottie_render.to_string(),
        )
        .json(&json!(lottie))
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await?)
}

pub async fn get_random_rule34(assyst: &Assyst, tags: &str) -> Result<Vec<Rule34Result>, Error> {
    Ok(assyst
        .reqwest_client
        .get(&*assyst.config.url.rule34)
        .query(&[
            ("tags", tags),
            ("limit", "10"),
            ("rating", "e"),
            ("formats", "jpeg png gif jpg webm mp4"),
        ])
        .send()
        .await?
        .error_for_status()?
        .json::<Vec<Rule34Result>>()
        .await?)
}

#[derive(Clone, Eq, PartialEq)]
pub enum ServiceStatus {
    Online(usize /* time to respond */),
    Offline,
}
impl ToString for ServiceStatus {
    fn to_string(&self) -> String {
        match self {
            ServiceStatus::Online(time) => format!("{} ({}ms)", "Online".fg_green(), time),
            ServiceStatus::Offline => "Offline".fg_red(),
        }
    }
}

#[derive(Clone)]
pub struct HealthcheckResult {
    pub service: String,
    pub status: ServiceStatus,
}
impl HealthcheckResult {
    pub fn new(service: String, status: ServiceStatus) -> Self {
        Self { service, status }
    }

    pub fn new_from_result<T, E>(service: &str, result: Result<T, E>, time: usize) -> Self {
        match result {
            Ok(_) => Self::new(service.to_string(), ServiceStatus::Online(time)),
            Err(_) => Self::new(service.to_string(), ServiceStatus::Offline),
        }
    }
}

pub async fn healthcheck(assyst: Arc<Assyst>) -> Vec<HealthcheckResult> {
    let mut results = Vec::<HealthcheckResult>::new();

    let timer = Instant::now();

    let wsi_result = run_wsi_job(
        assyst.clone(),
        FifoSend::Stats(FifoData::new(vec![], NoneQuery {})),
        UserId::new(1),
    )
    .await;
    results.push(HealthcheckResult::new_from_result(
        "WSI",
        wsi_result,
        timer.elapsed().as_millis() as _,
    ));

    let timer = Instant::now();
    let rule34_result = get_random_rule34(&*assyst, "").await;
    results.push(HealthcheckResult::new_from_result(
        "Rule 34",
        rule34_result,
        timer.elapsed().as_millis() as _,
    ));

    let timer = Instant::now();
    let burntext_result = burning_text("a").await;
    results.push(HealthcheckResult::new_from_result(
        "Burntext",
        burntext_result,
        timer.elapsed().as_millis() as _,
    ));

    let timer = Instant::now();
    let fake_eval_result = fake_eval(&assyst, "1", false).await;
    results.push(HealthcheckResult::new_from_result(
        "Eval",
        fake_eval_result,
        timer.elapsed().as_millis() as _,
    ));

    let timer = Instant::now();
    let rust_result = rust::run_binary(
        &assyst.reqwest_client,
        "1",
        "stable",
        OptimizationLevel::Debug,
    )
    .await;
    results.push(HealthcheckResult::new_from_result(
        "Rust",
        rust_result,
        timer.elapsed().as_millis() as _,
    ));

    let timer = Instant::now();
    let ocr_result = ocr_image(&assyst.reqwest_client, "https://i.jacher.io/ab.png").await;
    results.push(HealthcheckResult::new_from_result(
        "OCR",
        ocr_result,
        timer.elapsed().as_millis() as _,
    ));

    let timer = Instant::now();
    let identify_result =
        identify::identify_image(&assyst.reqwest_client, "https://i.jacher.io/ab.png", "").await;
    results.push(HealthcheckResult::new_from_result(
        "Identify",
        identify_result,
        timer.elapsed().as_millis() as _,
    ));

    let timer = Instant::now();
    let patreon_result = patreon::get_patrons(assyst.clone(), &assyst.config.auth.patreon).await;
    results.push(HealthcheckResult::new_from_result(
        "Patreon",
        patreon_result,
        timer.elapsed().as_millis() as _,
    ));

    let timer = Instant::now();
    let filer_result = upload_to_filer(assyst.clone(), Bytes::from(vec![1]), "text/plain").await;
    results.push(HealthcheckResult::new_from_result(
        "Filer",
        filer_result,
        timer.elapsed().as_millis() as _,
    ));

    let timer = Instant::now();
    let char_result = get_char_info(&assyst.reqwest_client, 'a').await;
    results.push(HealthcheckResult::new_from_result(
        "Char Info",
        char_result,
        timer.elapsed().as_millis() as _,
    ));

    let timer = Instant::now();
    let bt_result = bt::bad_translate(&assyst.reqwest_client, "a").await;
    results.push(HealthcheckResult::new_from_result(
        "Bad Translate",
        bt_result,
        timer.elapsed().as_millis() as _,
    ));

    let timer = Instant::now();
    let database_result = assyst.database.get_command_usage_stats().await;
    results.push(HealthcheckResult::new_from_result(
        "Database",
        database_result,
        timer.elapsed().as_millis() as _,
    ));

    let status = downloader::healthcheck(&assyst).await;
    results.push(HealthcheckResult::new("Content Proxy".into(), status));

    results
}
