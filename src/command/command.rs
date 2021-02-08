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

    pub fn without_reply(text: String) -> Self {
        CommandParseError {
            error: text,
            should_reply: false
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

pub mod must {
    use super::ParsedArgument;

    pub fn either<'a>(actual: &ParsedArgument, options: &'a [&'a str]) -> Result<&'a &'a str, String> {
        let as_string = match actual {
            ParsedArgument::Text(data) => data,
            _ => return Err("expected text argument".to_owned())
        };

        options.iter().find(|&&option| option == as_string).ok_or("argument not found in options".to_owned())
    }
}