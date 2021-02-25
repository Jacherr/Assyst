use reqwest::{Client, Error as ReqwestError};

const API_BASE: &str = "http://translate.y21_.repl.co";
const MAX_ATTEMPTS: u8 = 5;

pub enum TranslateError {
    Reqwest(ReqwestError),
    Raw(&'static str)
}

async fn translate_retry(client: &Client, text: &str) -> Result<String, TranslateError> {
    client
        .get(API_BASE)
        .query(&[("text", text)])
        .send()
        .await
        .map_err(TranslateError::Reqwest)?
        .text()
        .await
        .map_err(TranslateError::Reqwest)
}

pub async fn translate(
    client: &Client,
    text: &str
) -> Result<String, TranslateError> {
    let mut attempt = 0;

    while attempt <= MAX_ATTEMPTS {
        let result = translate_retry(client, text).await?;

        // A bit hacky but if it starts with <!doctype html>, we're assuming we got HTML as response
        // which probably means the proxy failed for an unknown reason
        // in which case we're not going to return and keep retrying
        if !result.starts_with("<!doctype html>") {
            return Ok(result)
        }

        eprintln!("Proxy failed! Raw response: {}", result);

        attempt += 1;
    }

    Err(TranslateError::Raw("BT Failed: Too many attempts"))
}
