use super::command::{Command, ParsedArgument, ParsedCommand};
use super::categories::{misc::*, image::*};
use std::{collections::HashMap, pin::Pin};
use crate::command::context::Context;
use std::future::Future;
use std::sync::Arc;

pub type CommandResult = Result<(), String>;
pub type CommandResultOuter = Pin<Box<dyn Future<Output = CommandResult> + Send>>;
pub type CommandRun = Box<dyn Fn(Arc<Context>, Vec<ParsedArgument>) -> CommandResultOuter + Send + Sync>;

macro_rules! register_command {
    ($self:expr, $command:expr, $run_fn:expr) => {{
        // Registering the same command for each alias is fine because it will point to the same object
        for alias in &$command.aliases {
            $self.commands.insert(alias, &*$command);
        }

        $self.commands.insert(&$command.name, &*$command);
        $self.command_runs.insert(&$command.name, Box::new(move |context, args| Box::pin($run_fn(context, args))));
    }}
}

pub struct CommandRegistry {
    pub command_runs: HashMap<&'static str, CommandRun>,
    pub commands: HashMap<&'static str, &'static Command>
}

impl CommandRegistry {
    pub fn new() -> Self {
        CommandRegistry {
            command_runs: HashMap::new(),
            commands: HashMap::new()
        }
    }

    pub async fn execute_command(&self, parsed_command: ParsedCommand, context: Arc<Context>) -> Result<(), String> {
        let command_run = self.command_runs.get(parsed_command.calling_name).unwrap();
        command_run(context, parsed_command.args).await
    }

    pub fn get_command_from_name_or_alias(&self, name: &str) -> Option<&'static Command> {
        self.commands.get(name).and_then(|command| Some(*command))
    }

    pub fn register_commands(&mut self) {
        register_command!(self, PING_COMMAND, run_ping_command);
        register_command!(self, ENLARGE_COMMAND, run_enlarge_command);
        register_command!(self, CAPTION_COMMAND, run_caption_command);
    }
}