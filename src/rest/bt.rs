use reqwest::{Client, Error as ReqwestError};
use serde::Deserialize;

const API_BASE: &str = "http://translate.y21_.repl.co";
const MAX_ATTEMPTS: u8 = 5;

#[derive(Debug)]
pub enum TranslateError {
    Reqwest(ReqwestError),
    Raw(&'static str),
}

#[derive(Deserialize)]
pub struct Translation {
    pub lang: String,
    pub text: String
}

#[derive(Deserialize)]
pub struct TranslateResult {
    pub translations: Vec<Translation>,
    pub result: Translation
}

async fn translate_retry(client: &Client, text: &str) -> Result<TranslateResult, TranslateError> {
    client
        .get(API_BASE)
        .query(&[("text", text)])
        .send()
        .await
        .map_err(TranslateError::Reqwest)?
        .json()
        .await
        .map_err(TranslateError::Reqwest)
}

pub async fn translate(client: &Client, text: &str) -> Result<TranslateResult, TranslateError> {
    let mut attempt = 0;

    while attempt <= MAX_ATTEMPTS {
        match translate_retry(client, text).await {
            Ok(result) => return Ok(result),
            Err(e) => eprintln!("Proxy failed! {:?}", e)
        };

        attempt += 1;
    }

    Err(TranslateError::Raw("BT Failed: Too many attempts"))
}
