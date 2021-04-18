use bytes::Bytes;
use serde::{Serialize, Deserialize};
use reqwest::{Client, Error};

pub mod annmarie;
pub mod bt;
pub mod rust;
pub mod wsi;
pub mod maryjane;

mod routes {
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
    pub code: String
}

#[derive(Deserialize)]
pub struct FakeEvalResponse {
    pub message: String
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

pub async fn fake_eval(
    client: &Client,
    code: &str
) -> Result<FakeEvalResponse, Error> {
    client.post(routes::FAKE_EVAL)
        .json(&FakeEvalBody {
            code: code.to_string()
        })
        .send()
        .await?
        .json()
        .await
}