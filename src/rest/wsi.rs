use bytes::Bytes;
use reqwest::Error;
use serde::Deserialize;

const API_BASE: &str = "https://wsi.jacher.io";

mod routes {
    pub const CAPTION: &str = "/caption";
}
#[derive(Deserialize)]
pub struct WsiError {
    pub code: u16,
    pub message: Box<str>
}

pub enum RequestError {
    Reqwest(Error),
    Wsi(WsiError)
}

pub async fn caption(client: &reqwest::Client, image: Bytes, text: &str) -> Result<Bytes, RequestError> {
    let result = client
        .post(&format!("{}{}", API_BASE, routes::CAPTION))
        .header(reqwest::header::AUTHORIZATION, "0192837465")
        .query(&[("text", text)])
        .body(image)
        .send()
        .await
        .map_err(|e| RequestError::Reqwest(e))?;
    return if result.status() != reqwest::StatusCode::OK {
        let json = result.json::<WsiError>().await
            .map_err(|err| RequestError::Reqwest(err))?;
        Err(RequestError::Wsi(json))
    } else {
        let bytes = result.bytes().await
            .map_err(|e| RequestError::Reqwest(e))?;
        Ok(bytes)
    }
}