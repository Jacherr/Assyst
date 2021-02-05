use reqwest::{Error, Client};

const API_BASE: &str = "http://translate.y21_.repl.co";

pub async fn translate(
    client: &Client,
    text: &str,
    // key: Option<&str> <-- TODO
) -> Result<String, Error> {
    client
        .get(API_BASE)
        .query(&[("text", text)])
        .send()
        .await?
        .text()
        .await
}