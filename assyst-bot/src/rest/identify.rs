use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::routes;

#[derive(Serialize)]
pub struct IdentifyBody<'a> {
    pub url: &'a str,
}

#[derive(Deserialize)]
pub struct IdentifyResponse {
    pub description: Option<IdentifyDescription>,
}

#[derive(Deserialize)]
pub struct IdentifyDescription {
    pub captions: Vec<IdentifyCaption>,
}

#[derive(Deserialize)]
pub struct IdentifyCaption {
    pub text: String,
    pub confidence: f32,
}

pub async fn identify_image(
    client: &Client,
    url: &str,
    api_key: &str,
) -> reqwest::Result<IdentifyResponse> {
    client
        .post(routes::IDENTIFY)
        .header("x-rapidapi-key", api_key)
        .json(&IdentifyBody { url })
        .send()
        .await?
        .json()
        .await
}
