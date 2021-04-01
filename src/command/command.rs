use bytes::Bytes;

#[derive(Debug)]
pub enum Argument {
    String,
    ImageUrl,
    ImageBuffer,
    StringRemaining,
    Choice(&'static [&'static str]),
    Optional(Box<Argument>),
    OptionalWithDefault(Box<Argument>, &'static str)
}
#[derive(Debug, PartialEq)]
pub enum ParsedArgument {
    Text(String),
    Binary(Bytes),
    Choice(&'static str),
    Nothing
}
impl ParsedArgument {
    pub fn is_nothing(&self) -> bool {
        *self == ParsedArgument::Nothing
    }
}

#[derive(PartialEq, Debug)]
pub enum CommandAvailability {
    Public,
    GuildOwner,
    Private,
}
impl CommandAvailability {
    pub fn to_string(&self) -> String {
        match self {
            CommandAvailability::Private => "Private".to_owned(),
            CommandAvailability::Public => "Public".to_owned(),
            CommandAvailability::GuildOwner => "Guild Owner".to_owned()
        }
    }
}

#[derive(Debug)]
pub enum CommandParseErrorType {
    MissingArgument,
    InvalidArgument,
    MediaDownloadFail,
    MissingPermissions,
    Other
}
#[derive(Debug)]
pub struct CommandParseError<'a> {
    pub error: String,
    pub should_reply: bool,
    pub command: Option<&'a Command>,
    pub error_type: CommandParseErrorType
}
impl<'a> CommandParseError<'a> {
    pub fn with_reply(text: String, command: Option<&'a Command>, r#type: CommandParseErrorType) -> Self {
        CommandParseError {
            error: text,
            should_reply: true,
            command,
            error_type: r#type
        }
    }

    pub fn without_reply(text: String, r#type: CommandParseErrorType) -> Self {
        CommandParseError {
            error: text,
            should_reply: false,
            command: None,
            error_type: r#type
        }
    }
}
#[derive(Debug)]
pub struct CommandMetadata {
    pub description: &'static str,
    pub examples: Vec<&'static str>,
    pub usage: &'static str,
}
#[derive(Debug)]
pub struct ParsedCommand {
    pub args: Vec<ParsedArgument>,
    pub calling_name: &'static str,
}

pub struct ParsedArgumentResult {
    pub should_increment_index: bool,
    pub should_break: bool,
    pub value: ParsedArgument,
}
impl ParsedArgumentResult {
    pub fn increment(value: ParsedArgument) -> Self {
        ParsedArgumentResult {
            should_increment_index: true,
            should_break: false,
            value,
        }
    }
    pub fn no_increment(value: ParsedArgument) -> Self {
        ParsedArgumentResult {
            should_increment_index: false,
            should_break: false,
            value,
        }
    }
    pub fn r#break(value: ParsedArgument) -> Self {
        ParsedArgumentResult {
            should_increment_index: false,
            should_break: true,
            value,
        }
    }
}
#[derive(Debug)]
pub struct Command {
    pub aliases: Vec<&'static str>,
    pub args: Vec<Argument>,
    pub availability: CommandAvailability,
    pub metadata: CommandMetadata,
    pub name: &'static str,
    pub cooldown_seconds: usize,
    pub category: &'static str
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
