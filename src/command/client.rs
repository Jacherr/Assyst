use twilight_model::channel::Message;
use std::sync::Arc;
use crate::Assyst;

use super::{command::ParsedCommand, registry::CommandRegistry};

pub fn get_command_name_from<'a>(content: &'a str, prefix: &str) -> Option<&'a str> {
    return if !content.starts_with(&prefix) || content.len() == prefix.len() {
        None
    } else {
        Some(&content[prefix.len()..])
    }
}
#[derive(Debug)]
pub struct CommandParseError {
    error: String,
    should_reply: bool
}
pub struct CommandClient {
    assyst: Option<Arc<Assyst>>,
    registry: CommandRegistry
}
impl CommandClient {
    pub fn new() -> Self {
        let mut command_client = CommandClient {
            assyst: None,
            registry: CommandRegistry::new()
        };
        command_client.registry.register_commands();
        command_client
    }

    pub fn set_assyst(&mut self, assyst: Arc<Assyst>) {
        self.assyst = Some(assyst);
    }

    pub fn get_assyst(&self) -> Arc<Assyst> {
        self.assyst.to_owned().expect("assyst isnt defined at READY")
    }

    pub async fn handle_command(&self, message: Message) -> Result<(), String> {
        let assyst = self.get_assyst();
        let try_prefix = assyst.database.get_or_set_prefix_for(
            message.guild_id.unwrap().0,
            &assyst.config.default_prefix)
            .await
            .map_err(|err| err.to_string())?;
        let prefix;

        match try_prefix {
            Some(p) => {
                prefix = p;
            },
            None => return Ok(())
        };

        let t_command = self.parse_command(message.content, &prefix).unwrap();
        let command;
        match t_command {
            Some(c) => command = c,
            None => return Ok(())
        };
        self.registry.execute_command(command).await;
        Ok(())
    }

    pub fn parse_command(&self, content: String, prefix: &str) -> Result<Option<ParsedCommand>, CommandParseError> {
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