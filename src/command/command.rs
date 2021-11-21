use std::{
    borrow::Cow,
    collections::HashMap,
    error::Error,
    fmt::{Display, Error as FmtError, Formatter},
    sync::Arc,
    time::Duration,
};

use bytes::Bytes;

use super::context::Context;

#[derive(Debug)]
pub enum FlagKind {
    Text,
    Number,
    Decimal,
    Boolean,
    Choice(&'static [&'static str]),
}

#[derive(Debug)]
pub enum ParsedFlagKind {
    Text(String),
    Number(u64),
    Decimal(f64),
    Boolean(bool),
}

impl ParsedFlagKind {
    pub fn as_text(&self) -> Cow<'_, str> {
        match self {
            Self::Text(t) => Cow::Borrowed(t),
            Self::Number(n) => Cow::Owned(n.to_string()),
            Self::Decimal(n) => Cow::Owned(n.to_string()),
            Self::Boolean(b) => Cow::Owned(b.to_string()),
        }
    }

    pub fn as_number(&self) -> Option<u64> {
        match self {
            Self::Number(n) => Some(*n),
            Self::Decimal(n) => Some(*n as u64),
            _ => None,
        }
    }

    pub fn as_decimal(&self) -> Option<f64> {
        match self {
            Self::Number(n) => Some(*n as f64),
            Self::Decimal(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            Self::Boolean(b) => Some(*b),
            _ => None,
        }
    }
}

pub type Flag = (&'static str, Option<FlagKind>);
pub type ParsedFlag = Option<ParsedFlagKind>;
pub type RawFlags = HashMap<&'static str, Flag>;
pub type ParsedFlags = HashMap<&'static str, ParsedFlag>;

#[derive(Debug)]
pub enum Argument {
    String,
    ImageUrl,
    ImageBuffer,
    StringRemaining,
    Integer,
    Decimal,
    Choice(&'static [&'static str]),
    Optional(Box<Argument>),
    OptionalWithDefault(Box<Argument>, &'static str),
    OptionalWithDefaultDynamic(Box<Argument>, fn(Arc<Context>) -> ParsedArgument),
}
#[derive(Debug, PartialEq, Clone)]
pub enum ParsedArgument {
    Text(String),
    Binary(Bytes),
    Choice(&'static str),
    Nothing,
}
impl ParsedArgument {
    pub fn is_nothing(&self) -> bool {
        *self == ParsedArgument::Nothing
    }
    pub fn as_text(&self) -> &str {
        match self {
            ParsedArgument::Text(t) => t,
            otherwise => panic!("expected text argument, got {:?}", otherwise),
        }
    }
    pub fn maybe_text(&self) -> Option<&str> {
        match self {
            ParsedArgument::Text(t) => Some(t),
            _ => None,
        }
    }
    pub fn into_bytes(self) -> Bytes {
        match self {
            ParsedArgument::Binary(t) => t,
            otherwise => panic!("expected buffer argument, got {:?}", otherwise),
        }
    }
    pub fn as_bytes(&self) -> Bytes {
        match self {
            ParsedArgument::Binary(t) => t.clone(),
            otherwise => panic!("expected buffer argument, got {:?}", otherwise),
        }
    }
    pub fn as_choice(&self) -> &str {
        match self {
            ParsedArgument::Choice(t) => *t,
            otherwise => panic!("expected choice argument, got {:?}", otherwise),
        }
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
            CommandAvailability::GuildOwner => "Guild Owner".to_owned(),
        }
    }
}

#[derive(Debug)]
pub enum CommandParseErrorType {
    MissingArgument,
    InvalidArgument,
    MediaDownloadFail,
    MissingPermissions,
    Other,
}
#[derive(Debug)]
pub struct CommandParseError<'a> {
    pub error: String,
    pub should_reply: bool,
    pub command: Option<&'a Command>,
    pub error_type: CommandParseErrorType,
}
impl<'a> CommandParseError<'a> {
    pub fn with_reply(
        text: String,
        command: Option<&'a Command>,
        r#type: CommandParseErrorType,
    ) -> Self {
        CommandParseError {
            error: text,
            should_reply: true,
            command,
            error_type: r#type,
        }
    }

    pub fn without_reply(text: String, r#type: CommandParseErrorType) -> Self {
        CommandParseError {
            error: text,
            should_reply: false,
            command: None,
            error_type: r#type,
        }
    }

    pub fn permission_validator_failed() -> Self {
        CommandParseError::with_reply(
            "Permission validator failed".to_owned(),
            None,
            CommandParseErrorType::MissingPermissions,
        )
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
    pub flags: ParsedFlags,
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
    pub flags: RawFlags,
    pub availability: CommandAvailability,
    pub metadata: CommandMetadata,
    pub name: &'static str,
    pub cooldown_seconds: usize,
    pub category: &'static str,
    pub disabled: bool,
}

pub struct CommandBuilder {
    aliases: Vec<&'static str>,
    args: Vec<Argument>,
    availability: Option<CommandAvailability>,
    flags: RawFlags,
    metadata: CommandMetadata,
    name: &'static str,
    cooldown_seconds: Option<usize>,
    category: Option<&'static str>,
    disabled: bool,
}
impl CommandBuilder {
    pub fn new(name: &'static str) -> Self {
        Self {
            aliases: vec![],
            args: vec![],
            availability: None,
            flags: HashMap::new(),
            metadata: CommandMetadata {
                description: "",
                examples: vec![],
                usage: "",
            },
            name,
            cooldown_seconds: None,
            category: None,
            disabled: false,
        }
    }

    pub fn alias(mut self, alias: &'static str) -> Self {
        self.aliases.push(alias);
        self
    }

    pub fn arg(mut self, argument: Argument) -> Self {
        self.args.push(argument);
        self
    }

    pub fn availability(mut self, availability: CommandAvailability) -> Self {
        self.availability = Some(availability);
        self
    }

    pub fn category(mut self, category: &'static str) -> Self {
        self.category = Some(category);
        self
    }

    pub fn cooldown(mut self, cooldown: Duration) -> Self {
        self.cooldown_seconds = Some(cooldown.as_secs() as usize);
        self
    }

    pub fn description(mut self, description: &'static str) -> Self {
        self.metadata.description = description;
        self
    }

    pub fn disable(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub fn example(mut self, example: &'static str) -> Self {
        self.metadata.examples.push(example);
        self
    }

    pub fn flag(mut self, key: &'static str, ty: Option<FlagKind>) -> Self {
        self.flags.insert(key, (key, ty));
        self
    }

    pub fn public(mut self) -> Self {
        self.availability = Some(CommandAvailability::Public);
        self
    }

    pub fn usage(mut self, usage: &'static str) -> Self {
        self.metadata.usage = usage;
        self
    }

    pub fn build(self) -> Command {
        let category = self.category.expect("Command must belong to a category");
        let cooldown = self.cooldown_seconds.unwrap_or(4);
        let availability = self
            .availability
            .expect("Command must have a defined availability");
        let description = if self.metadata.description.is_empty() {
            panic!("Command must have a description")
        } else {
            self.metadata.description
        };

        Command {
            aliases: self.aliases,
            args: self.args,
            flags: self.flags,
            metadata: CommandMetadata {
                description,
                examples: self.metadata.examples,
                usage: self.metadata.usage,
            },
            name: self.name,
            availability,
            category,
            cooldown_seconds: cooldown,
            disabled
        }
    }
}

#[derive(Debug)]
pub struct CommandError {
    text: String,
}

impl CommandError {
    pub fn new(text: impl Into<String>) -> Self {
        CommandError { text: text.into() }
    }

    pub fn new_boxed(text: impl Into<String>) -> Box<Self> {
        Box::new(CommandError { text: text.into() })
    }
}

impl Error for CommandError {}

impl Display for CommandError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), FmtError> {
        write!(formatter, "Command Error: {}", self.text)
    }
}
