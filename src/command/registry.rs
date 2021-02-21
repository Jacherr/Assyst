use super::command::{Command, ParsedArgument, ParsedCommand};
use super::categories::{misc::*, image::*};
use std::{collections::HashMap, pin::Pin};
use crate::command::context::Context;
use std::future::Future;
use std::sync::Arc;
use futures::lock;
use tokio::{sync::{Mutex, oneshot}, time::{sleep, Duration}};

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
        let command_processed = Arc::new(Mutex::new(false));
        
        let command_processed_c = command_processed.clone();
        let context_c = context.clone();

        tokio::spawn(async move {
            sleep(Duration::from_millis(500)).await;
            let lock = *command_processed_c.lock().await;
            if lock == false {
                context_c.assyst.http.create_typing_trigger(context_c.message.channel_id).await
                    .unwrap();
            }
        });

        let command_run = self.command_runs.get(parsed_command.calling_name).unwrap();
        let result = command_run(context, parsed_command.args).await;
        let mut lock = command_processed.lock().await;
        *lock = true;
        result
    }

    pub fn get_command_from_name_or_alias(&self, name: &str) -> Option<&'static Command> {
        self.commands.get(name).and_then(|command| Some(*command))
    }

    pub fn register_commands(&mut self) {
        //register_command!(self, _3D_ROTATE_COMMAND, run_3d_rotate_command);
        register_command!(self, PING_COMMAND, run_ping_command);
        register_command!(self, STATS_COMMAND, run_stats_command);
        register_command!(self, ENLARGE_COMMAND, run_enlarge_command);
        register_command!(self, CAPTION_COMMAND, run_caption_command);
        register_command!(self, GIF_SPEED_COMMAND, run_gif_speed_command);
        register_command!(self, HELP_COMMAND, run_help_command);
        register_command!(self, INVITE_COMMAND, run_invite_command);
        register_command!(self, IMAGEMAGICK_EVAL_COMMAND, run_imagemagick_eval_command);
        register_command!(self, MELT_COMMAND, run_melt_command);
        register_command!(self, MOTIVATE_COMMAND, run_motivate_command);
        register_command!(self, RAINBOW_COMMAND, run_rainbow_command);
        register_command!(self, REVERSE_COMMAND, run_reverse_command);
        register_command!(self, SPIN_COMMAND, run_spin_command);
        register_command!(self, WORMHOLE_COMMAND, run_wormhole_command);
        register_command!(self, ZOOM_COMMAND, run_zoom_command);
    }
}