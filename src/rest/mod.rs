use bytes::Bytes;
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};

pub mod annmarie;
pub mod bt;
pub mod maryjane;
pub mod rust;
pub mod wsi;

mod routes {
    pub const COOL_TEXT: &str = "https://cooltext.com/PostChange";
    pub const CDN: &str = "https://cdn.jacher.io";
    pub const OCR: &str = "http://ocr.y21_.repl.co/?url=";
    pub const CHARINFO: &str = "https://www.fileformat.info/info/unicode/char/";
    pub const FAKE_EVAL: &str = "https://jacher.io/eval";
}

pub enum OcrError {
    NetworkError,
    HtmlResponse,
}

impl ToString for OcrError {
    fn to_string(&self) -> String {
        match self {
            Self::NetworkError => "An unknown network error occurred".to_string(),
            Self::HtmlResponse => "Failed to parse response".to_string(),
        }
    }
}

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
    pub is_animated: bool
}

pub async fn ocr_image(client: &Client, url: &str) -> Result<String, OcrError> {
    let text = client
        .get(&format!("{}{}", routes::OCR, url))
        .send()
        .await
        .map_err(|_| OcrError::NetworkError)?
        .text()
        .await
        .map_err(|_| OcrError::NetworkError)?;

    if text.to_ascii_lowercase().starts_with("<!doctype html>") {
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

pub async fn burning_text(client: &Client, text: &str) -> Result<Bytes, Error> {
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
            ("BackgroundColor_color", "#FFFFFF")
        ])
        .header("content-length", "0")
        .send()
        .await?
        .json::<CoolTextResponse>()
        .await?;

    let url = cool_text_response.render_location;
    let content = client.get(&url.replace("https", "http")).send().await?.bytes().await?;

    Ok(content)
}