use bytes::Bytes;
#[derive(Debug)]
pub enum Argument {
    String,
    ImageUrl,
    ImageBuffer,
    StringRemaining,
    Choice(&'static [&'static str]),
}
#[derive(Debug)]
pub enum ParsedArgument {
    Text(String),
    Binary(Bytes),
    Choice(&'static str),
}
#[derive(PartialEq, Debug)]
pub enum CommandAvailability {
    Public,
    RequiresPermission(u16),
    GuildOwner,
    Private,
}
#[derive(Debug)]
pub struct CommandParseError<'a> {
    pub error: String,
    pub should_reply: bool,
    pub command: Option<&'a Command>,
}
impl<'a> CommandParseError<'a> {
    pub fn set_command(&mut self, command: &'a Command) -> &mut Self {
        self.command = Some(command);
        self
    }

    pub fn with_reply(text: String, command: Option<&'a Command>) -> Self {
        CommandParseError {
            error: text,
            should_reply: true,
            command,
        }
    }

    pub fn without_reply(text: String) -> Self {
        CommandParseError {
            error: text,
            should_reply: false,
            command: None,
        }
    }
}
#[derive(Debug)]
pub struct CommandMetadata {
    pub description: Box<str>,
    pub examples: Vec<Box<str>>,
    pub usage: Box<str>,
}
#[derive(Debug)]
pub struct ParsedCommand {
    pub args: Vec<ParsedArgument>,
    pub calling_name: &'static str,
}

pub struct ParsedArgumentResult {
    pub should_increment_index: bool,
    pub value: ParsedArgument,
}
impl ParsedArgumentResult {
    pub fn increment(value: ParsedArgument) -> Self {
        ParsedArgumentResult {
            should_increment_index: true,
            value,
        }
    }
    pub fn no_increment(value: ParsedArgument) -> Self {
        ParsedArgumentResult {
            should_increment_index: false,
            value,
        }
    }
}
#[derive(Debug)]
pub struct Command {
    pub aliases: Vec<Box<str>>,
    pub args: Vec<Argument>,
    pub availability: CommandAvailability,
    pub metadata: CommandMetadata,
    pub name: Box<str>,
}

pub mod force_as {
    use bytes::Bytes;

    use super::ParsedArgument;

    pub fn image_buffer(argument: ParsedArgument) -> Bytes {
        match argument {
            ParsedArgument::Binary(data) => data,
            _ => panic!("expected buffer argument"),
        }
    }

    pub fn text(argument: &ParsedArgument) -> &str {
        match argument {
            ParsedArgument::Text(data) => data,
            _ => panic!("expected text argument"),
        }
    }

    pub fn choice(argument: &ParsedArgument) -> &'static str {
        match argument {
            ParsedArgument::Choice(data) => data,
            _ => panic!("expected choice, got {:?}", argument),
        }
    }
}
