use bytes::Bytes;
pub enum Argument {
    String,
    ImageUrl,
    ImageBuffer,
    StringRemaining
}
#[derive(Debug)]
pub enum ParsedArgument {
    Text(String),
    Binary(Bytes)
}
pub enum CommandAvailability {
    Public,
    RequiresPermission(u16),
    GuildOwner,
    Private
}
#[derive(Debug)]
pub struct CommandParseError {
    pub error: String,
    pub should_reply: bool
}
impl CommandParseError {
    pub fn with_reply(text: String) -> Self {
        CommandParseError {
            error: text,
            should_reply: true
        }
    }
}
pub struct CommandMetadata {
    pub description: Box<str>,
    pub examples: Vec<Box<str>>,
    pub usage: Box<str>
}
#[derive(Debug)]
pub struct ParsedCommand {
    pub args: Vec<ParsedArgument>,
    pub calling_name: &'static str
}

pub struct ParsedArgumentResult {
    pub should_increment_index: bool,
    pub value: ParsedArgument
}
impl ParsedArgumentResult {
    pub fn increment(value: ParsedArgument) -> Self {
        ParsedArgumentResult {
            should_increment_index: true,
            value
        }
    }
    pub fn no_increment(value: ParsedArgument) -> Self {
        ParsedArgumentResult {
            should_increment_index: false,
            value
        }
    }
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

pub mod force_as {
    use bytes::Bytes;

    use super::ParsedArgument;

    pub fn image_buffer(argument: ParsedArgument) -> Bytes {
        match argument {
            ParsedArgument::Binary(data) => data,
            _ => panic!("expected buffer argument")
        }
    }

    pub fn text(argument: &ParsedArgument) -> &str {
        match argument {
            ParsedArgument::Text(data) => data,
            _ => panic!("expected text argument")
        }
    }
}