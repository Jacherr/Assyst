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
    applicaton: ApplicationApp,
    bot: ApplicationBot,
}

#[derive(Deserialize)]
pub struct ApplicationBot {
    id: usize,
    name: String,
    icon: Option<String>,
    description: Option<String>,
    summary: Option<String>,
    bot_public: bool,
    bot_require_code_grant: bool,
}

#[derive(Deserialize)]
pub struct ApplicationApp {
    id: usize,
    username: String,
    avatar: Option<String>,
    discriminator: u16,
    flags: u32,
    bot: bool,
    guilds: usize,
}

pub async fn get<'a, T: DeserializeOwned>(
    assyst: Arc<Assyst>,
    route: &str,
    query: &[(&str, &str)],
) -> Result<T, RequestError> {
    let result = assyst
        .reqwest_client
        .post(&format!("{}{}", assyst.config.maryjane_url, route))
        .header(
            reqwest::header::AUTHORIZATION,
            assyst.config.annmarie_auth.as_ref(),
        )
        .query(query)
        .send()
        .await
        .map_err(|_| RequestError::Reqwest("A network error occurred".to_owned()))?;

    result
        .json::<T>()
        .await
        .map_err(|e| RequestError::Reqwest(e.to_string()))
}

pub async fn get_application(assyst: Arc<Assyst>, id: &str) -> Result<Application, RequestError> {
    get(assyst, &parse_path_parameter(routes::APPLICATION.to_owned(), ("id", id)), &[]).await
}