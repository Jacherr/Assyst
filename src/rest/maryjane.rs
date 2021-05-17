use std::sync::Arc;

use serde::de::DeserializeOwned;
use serde::Deserialize;

use crate::{assyst::Assyst, rest::annmarie::RequestError};

use super::parse_path_parameter;

mod routes {
    pub const APPLICATION: &str = "/applications/:id";
}

#[derive(Deserialize)]
pub struct Application {
    pub application: ApplicationApp,
    pub bot: ApplicationBot,
}

#[derive(Deserialize)]
pub struct ApplicationApp {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub description: Option<String>,
    pub summary: Option<String>,
    pub bot_public: bool,
    pub bot_require_code_grant: bool,
}

#[derive(Deserialize)]
pub struct ApplicationBot {
    pub id: String,
    pub username: String,
    pub avatar: Option<String>,
    pub discriminator: String,
    pub flags: u32,
    pub bot: bool,
    pub guild_count: usize,
}

pub async fn get<'a, T: DeserializeOwned>(
    assyst: Arc<Assyst>,
    route: &str,
    query: &[(&str, &str)],
) -> Result<T, RequestError> {
    let url = format!("{}{}", assyst.config.maryjane_url, route);

    let result = assyst
        .reqwest_client
        .get(&url)
        .header(
            reqwest::header::AUTHORIZATION,
            assyst.config.maryjane_auth.as_ref(),
        )
        .query(query)
        .send()
        .await
        .map_err(|_| RequestError::Reqwest("A network error occurred".to_owned()))?;

    let status = result.status();
    if status != reqwest::StatusCode::OK {
        return Err(RequestError::InvalidStatus(status));
    };

    result
        .json::<T>()
        .await
        .map_err(|e| RequestError::Reqwest(e.to_string()))
}

pub async fn get_application(assyst: Arc<Assyst>, id: u64) -> Result<Application, RequestError> {
    get::<Application>(
        assyst,
        &parse_path_parameter(routes::APPLICATION.to_owned(), ("id", &id.to_string())),
        &[],
    )
    .await
}
