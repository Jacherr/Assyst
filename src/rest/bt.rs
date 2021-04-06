use reqwest::{Client, Error as ReqwestError};
use serde::Deserialize;

const API_BASE: &str = "http://translate.y21_.repl.co";
const MAX_ATTEMPTS: u8 = 5;

#[derive(Debug)]
pub enum TranslateError {
    Reqwest(ReqwestError),
    Raw(&'static str),
}

impl ToString for TranslateError {
    fn to_string(&self) -> String {
        match self {
            Self::Reqwest(_) => "A network error occurred".to_owned(),
            Self::Raw(r) => r.to_string()
        }
    }
}

#[derive(Deserialize)]
pub struct Translation {
    pub lang: String,
    pub text: String,
}

#[derive(Deserialize)]
pub struct TranslateResult {
    pub translations: Vec<Translation>,
    pub result: Translation,
}

async fn translate_retry(
    client: &Client,
    text: &str,
    target: Option<&str>,
    count: Option<u32>
) -> Result<TranslateResult, TranslateError> {
    let mut query_args = vec![("text", text.to_owned())];

    if let Some(target) = target {
        query_args.push(("target", target.to_owned()));
    }

    if let Some(count) = count {
        query_args.push(("count", count.to_string()));
    }

    client
        .get(API_BASE)
        .query(&query_args)
        .send()
        .await
        .map_err(TranslateError::Reqwest)?
        .json()
        .await
        .map_err(TranslateError::Reqwest)
}

async fn translate(
    client: &Client,
    text: &str,
    target: Option<&str>,
    count: Option<u32>
) -> Result<TranslateResult, TranslateError> {
    let mut attempt = 0;

    while attempt <= MAX_ATTEMPTS {
        match translate_retry(client, text, target, count).await {
            Ok(result) => return Ok(result),
            Err(e) => eprintln!("Proxy failed! {:?}", e),
        };

        attempt += 1;
    }

    Err(TranslateError::Raw("BT Failed: Too many attempts"))
}

pub async fn bad_translate(client: &Client, text: &str) -> Result<TranslateResult, TranslateError> {
    translate(client, text, None, None).await
}

pub async fn bad_translate_with_count(client: &Client, text: &str, count: u32) -> Result<TranslateResult, TranslateError> {
    translate(client, text, None, Some(count)).await
}

pub async fn translate_single(client: &Client, text: &str, target: &str) -> Result<TranslateResult, TranslateError> {
    translate(client, text, Some(target), Some(1)).await
}