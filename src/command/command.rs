use std::{collections::HashMap, sync::Arc, time::Duration};

use bytes::Bytes;

use super::context::Context;

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
#[derive(Debug, PartialEq)]
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
    pub category: &'static str,
}

pub struct CommandBuilder {
    aliases: Vec<&'static str>,
    args: Vec<Argument>,
    availability: Option<CommandAvailability>,
    metadata: CommandMetadata,
    name: &'static str,
    cooldown_seconds: Option<usize>,
    category: Option<&'static str>,
}
impl CommandBuilder {
    pub fn new(name: &'static str) -> Self {
        Self {
            aliases: vec![],
            args: vec![],
            availability: None,
            metadata: CommandMetadata {
                description: "",
                examples: vec![],
                usage: "",
            },
            name,
            cooldown_seconds: None,
            category: None,
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

    pub fn example(mut self, example: &'static str) -> Self {
        self.metadata.examples.push(example);
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
            metadata: CommandMetadata {
                description,
                examples: self.metadata.examples,
                usage: self.metadata.usage,
            },
            name: self.name,
            availability,
            category,
            cooldown_seconds: cooldown,
        }
    }
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

pub enum FlagInnerType {
    Boolean,
    String,
    Integer,
    Decimal,
    Choice(&'static [&'static str]),
}

pub type Flags = HashMap<&'static str, Option<&'static str>>;
