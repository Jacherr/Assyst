use super::command::{Command, ParsedArgument, ParsedCommand};
use super::categories::misc::*;
use std::{collections::HashMap, pin::Pin};
use crate::{box_str, command::context::Context};
use std::future::Future;
use std::sync::Arc;

pub type CommandResult = Result<(), String>;
pub type CommandResultOuter = Pin<Box<dyn Future<Output = CommandResult> + Send>>;
pub type CommandRun = Box<dyn Fn(Arc<Context>, Vec<ParsedArgument>) -> CommandResultOuter + Send + Sync>;

macro_rules! register_command {
    ($self:expr, $command:expr, $run_fn:expr) => {{
        $self.commands.insert($command.name.clone(), &*$command);
        $self.command_runs.insert($command.name.clone(), Box::new(move |context, args| Box::pin($run_fn(context, args))));
    }}
}

pub struct CommandRegistry {
    pub command_runs: HashMap<Box<str>, CommandRun>,
    pub commands: HashMap<Box<str>, &'static Command>
}
impl CommandRegistry {
    pub fn new() -> Self {
        CommandRegistry {
            command_runs: HashMap::new(),
            commands: HashMap::new()
        }
    }

    pub async fn execute_command(&self, parsed_command: ParsedCommand, context: Arc<Context>) -> Result<(), String> {
        let command_run = self.command_runs.get(&parsed_command.calling_name).unwrap();
        command_run(context, parsed_command.args).await
    }

    pub fn get_command_from_name_or_alias(&self, name: &str) -> Option<&'static Command> {
        let command = self.commands.get(&box_str!(name));
        return match command {
            Some(c) => Some(*c),
            None => {
                let valid_command = self.commands.values().find(|c| (**c).names().contains(&name));
                match valid_command {
                    Some(c) => Some(*c),
                    None => None
                }
            }
        }
    }

    pub fn register_commands(&mut self) {
        register_command!(self, PING_COMMAND, run_ping_command);
        register_command!(self, TEST_COMMAND, run_test_command);
    }
}