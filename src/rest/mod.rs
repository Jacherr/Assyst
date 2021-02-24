use bytes::Bytes;
use reqwest::{Error, Client};

pub mod wsi;
pub mod bt;
pub mod rust;

const API_BASE: &str = "https://cdn.jacher.io";

pub async fn upload_to_filer(client: &Client, data: Bytes, content_type: &str) -> Result<String, Error> {
    client
        .post(API_BASE)
        .header(reqwest::header::CONTENT_TYPE, content_type)
        .header(reqwest::header::AUTHORIZATION, "0192837465")
        .body(data)
        .send()
        .await?
        .text()
        .await
}