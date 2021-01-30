use super::context::Context;

pub enum Argument {
    String,
    ImageAny,
    StringRemaining
}

pub enum CommandAvailability {
    Public,
    RequiresPermission(u16),
    GuildOwner,
    Private
}
#[derive(Debug)]
pub struct CommandParseError {
    error: String,
    should_reply: bool
}

pub struct CommandMetadata {
    pub description: Box<str>,
    pub examples: Vec<Box<str>>,
    pub usage: Box<str>
}
#[derive(Debug)]
pub struct ParsedCommand {
    pub args: Vec<String>,
    pub calling_name: Box<str>
}
pub struct Command {
    pub aliases: Vec<Box<str>>,
    pub args: Vec<Argument>,
    pub availability: CommandAvailability,
    pub metadata: CommandMetadata,
    pub name: Box<str>
}
impl Command {
    pub fn names(&self) -> Vec<&str> {
        let mut new_vec: Vec<&str> = Vec::with_capacity(self.aliases.len() + 1);
        for alias in &self.aliases {
            new_vec.push(&alias);
        }
        new_vec.push(&self.name);
        new_vec
    }
}
