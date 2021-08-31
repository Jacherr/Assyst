use std::{collections::HashSet, fs::read_to_string};

use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Auth {
    pub annmarie: Box<str>,
    pub bot_list_webhook: Box<str>,
    pub discord_bot_list_post_stats: Box<str>,
    pub lottie_render: Box<str>,
    pub maryjane: Box<str>,
    pub patreon: Box<str>,
    pub top_gg_post_stats: Box<str>,
    pub wsi: Box<str>,
}

#[derive(Clone, Deserialize)]
pub struct Database {
    pub username: Box<str>,
    pub password: Box<str>,
    pub host: Box<str>,
    pub port: u16,
    pub database: Box<str>,
}
impl Database {
    pub fn to_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }
}

#[derive(Clone, Deserialize)]
pub struct Logs {
    pub fatal: String,
    pub info: String,
    pub vote: String,
    pub panic_notify_role: u64,
}

#[derive(Clone, Deserialize)]
pub struct Prefix {
    pub default: Box<str>,
    pub r#override: Box<str>,
}

#[derive(Clone, Deserialize)]
pub struct Url {
    pub annmarie: Box<str>,
    pub lottie_render: Box<str>,
    pub maryjane: Box<str>,
    pub wsi: Box<str>,
}

#[derive(Clone, Deserialize)]
pub struct User {
    pub admins: HashSet<u64>,
    pub blacklist: HashSet<u64>,
}

#[derive(Clone, Deserialize)]
pub struct Config {
    pub auth: Auth,
    pub bot_id: u64,
    pub bot_list_port: u16,
    pub database: Database,
    pub disable_bad_translator: bool,
    pub disable_reminder_check: bool,
    pub logs: Logs,
    pub prefix: Prefix,
    pub url: Url,
    pub user: User,
}
impl Config {
    pub fn new() -> Self {
        let buffer = read_to_string("./config.toml").unwrap();
        toml::from_str(&buffer).unwrap()
    }
}
