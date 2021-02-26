use std::borrow::Borrow;

use bytes::Bytes;
use reqwest::{Client, Error};

pub mod bt;
pub mod rust;
pub mod wsi;

mod routes {
    pub const CDN: &str = "https://cdn.jacher.io";
    pub const OCR: &str = "http://ocr--y21_.repl.co/?url=";
}

pub async fn ocr_image(client: &Client, url: &str) -> Result<String, Error> {
    client
        .get(&format!("{}{}", routes::OCR, url))
        .send()
        .await?
        .text()
        .await
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
