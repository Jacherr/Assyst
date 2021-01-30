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

pub struct CommandMetadata {
    pub description: Box<str>,
    pub examples: Vec<Box<str>>,
    pub usage: Box<str>
}
pub struct ParsedCommand {
    pub args: Vec<Box<str>>,
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
