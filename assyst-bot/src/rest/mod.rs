use std::fmt::Display;

use bytes::Bytes;
use reqwest::{Client, ClientBuilder, Error};
use serde::{Deserialize, Serialize};
use serde_json::json;

use std::error::Error as StdError;

use crate::assyst::Assyst;

pub mod annmarie;
pub mod bt;
pub mod codesprint;
pub mod identify;
pub mod maryjane;
pub mod patreon;
pub mod rust;
pub mod wsi;

mod routes {
    use assyst_common::consts::BOT_ID;

    pub const COOL_TEXT: &str = "https://cooltext.com/PostChange";
    pub const CDN: &str = "https://cdn.jacher.io";
    pub const OCR: &str = "http://ocr.y21_.repl.co/?url=";
    pub const CHARINFO: &str = "https://www.fileformat.info/info/unicode/char/";
    pub const FAKE_EVAL: &str = "https://jacher.io/eval";
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

#[derive(Serialize)]
struct FakeEvalBody {
    pub code: String,
}

#[derive(Deserialize)]
pub struct FakeEvalResponse {
    pub message: String,
}

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
    let mut text = client
        .get(format!("{}{}", routes::OCR, url))
        .send()
        .await
        .map_err(|_| OcrError::NetworkError)?
        .text()
        .await
        .map_err(|_| OcrError::NetworkError)?;
    text.make_ascii_lowercase();

    if text.starts_with("<!doctype html>") {
        return Err(OcrError::HtmlResponse);
    }

    return Ok(text);
}

pub async fn upload_to_filer(
    client: &Client,
    data: Bytes,
    content_type: &str,
) -> Result<String, Error> {
    client
        .post(routes::CDN)
        .header(reqwest::header::CONTENT_TYPE, content_type)
        .header(reqwest::header::AUTHORIZATION, "0192837465")
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

pub async fn fake_eval(client: &Client, code: &str) -> Result<FakeEvalResponse, Error> {
    client
        .post(routes::FAKE_EVAL)
        .json(&FakeEvalBody {
            code: code.to_string(),
        })
        .send()
        .await?
        .json()
        .await
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
    discord_bot_list_token: &str,
    top_gg_token: &str,
    discords_token: &str,
    guild_count: u32,
) -> Result<(), Error> {
    client
        .post(routes::discord_bot_list_stats_url())
        .header("authorization", discord_bot_list_token)
        .json(&json!({ "guilds": guild_count }))
        .send()
        .await?
        .error_for_status()?;

    client
        .post(routes::top_gg_stats_url())
        .header("authorization", top_gg_token)
        .json(&json!({ "server_count": guild_count }))
        .send()
        .await?
        .error_for_status()?;

    client
        .post(routes::discords_stats_url())
        .header("authorization", discords_token)
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
            ("limit", "1"),
            ("rating", "e"),
            ("formats", "jpeg png gif jpg webm mp4"),
        ])
        .send()
        .await?
        .error_for_status()?
        .json::<Vec<Rule34Result>>()
        .await?)
}
