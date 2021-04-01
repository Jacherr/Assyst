use bytes::Bytes;
use reqwest::{Client, Error};

pub mod bt;
pub mod rust;
pub mod wsi;
pub mod annmarie;

mod routes {
    pub const CDN: &str = "https://cdn.jacher.io";
    pub const OCR: &str = "http://ocr.y21_.repl.co/?url=";
}

pub enum OcrError {
    NetworkError,
    HtmlResponse
}

impl ToString for OcrError {
    fn to_string(&self) -> String {
        match self {
            Self::NetworkError => "An unknown network error occurred".to_string(),
            Self::HtmlResponse => "Failed to parse response".to_string()
        }
    }
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

    return Ok(text)
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
