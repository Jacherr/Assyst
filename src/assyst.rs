use std::fs::read_to_string;
use crate::command::{context::Context, registry::CommandRegistry};
use crate::database::Database;
use twilight_http::Client as HttpClient;
use serde::Deserialize;
use twilight_model::channel::Message;
use crate::command::command::{ParsedCommand, CommandParseError};
use std::sync::Arc;
#[derive(Clone, Deserialize)]
struct DatabaseInfo {
    username: Box<str>,
    password: Box<str>,
    host: Box<str>,
    port: u16,
    database: Box<str>,
}
impl DatabaseInfo {
    pub fn to_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }
}
#[derive(Clone, Deserialize)]
pub struct Config {
    database: DatabaseInfo,
    pub default_prefix: Box<str>,
}
impl Config {
    fn new() -> Self {
        let buffer = read_to_string("./config.toml").unwrap();
        toml::from_str(&buffer).unwrap()
    }
}

pub fn get_command_name_from<'a>(content: &'a str, prefix: &str) -> Option<&'a str> {
    return if !content.starts_with(&prefix) || content.len() == prefix.len() {
        None
    } else {
        Some(&content[prefix.len()..])
    }
}

pub struct Assyst {
    pub registry: CommandRegistry,
    pub config: Config,
    pub database: Database,
    pub http: HttpClient,
}
impl Assyst {
    pub async fn new(token: &str) -> Self {
        let http = HttpClient::new(token);
        let config = Config::new();
        let database = Database::new(2, config.database.to_url())
        .await
        .unwrap();
        let mut assyst = Assyst {
            registry: CommandRegistry::new(),
            config,
            database,
            http
        };
        assyst.registry.register_commands();
        assyst
    }

    pub async fn handle_command(self: &Arc<Self>, message: Message) -> Result<(), String> {
        let try_prefix = self.database.get_or_set_prefix_for(
            message.guild_id.unwrap().0,
            &self.config.default_prefix)
            .await
            .map_err(|err| err.to_string())?;
        let prefix;

        match try_prefix {
            Some(p) => {
                prefix = p;
            },
            None => return Ok(())
        };

        let t_command = self.parse_command(&message.content, &prefix).unwrap();
        let command;
        match t_command {
            Some(c) => command = c,
            None => return Ok(())
        };
        self.registry.execute_command(command, Context::new(self.clone(), message)).await;
        Ok(())
    }

    pub fn parse_command(&self, content: &str, prefix: &str) -> Result<Option<ParsedCommand>, CommandParseError> {
        let command_name = get_command_name_from(&content, &prefix)
            .unwrap_or("");
        let try_command = self.registry.get_command_from_name_or_alias(command_name);
        let command;
        match try_command {
            Some(c) => command = c,
            None => return Ok(None)
        };
        Ok(Some(ParsedCommand {
            calling_name: command.name.clone(),
            args: vec![]
        }))
    }
}