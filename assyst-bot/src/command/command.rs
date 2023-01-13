use std::time::Duration;

pub struct Command {
    pub name: String,
    /// Text commands only
    pub aliases: Vec<String>,
    /// Text commands only, in form: [option] <optional option> <-flag option>
    pub usage: String,
    /// Text commands only, only include arguments
    pub examples: Vec<String>,
    pub description: String,
    pub args: Vec<CommandArgument>,
    /// Slash commands: same as optional arguments
    pub flags: Vec<CommandFlag>,
    pub cooldown: Duration,
    pub access: CommandAccess,
    pub nsfw: bool,
    pub disabled: bool
}

pub struct CommandArgument {
    pub name: String,
    pub description: String,
    pub r#type: CommandArgumentType,
}
pub enum CommandArgumentType {
    /// String and StringRemaining are equals in slash commands, can be one word or "multiple words"
    String,
    /// String and StringRemaining are equals in slash commands
    StringRemaining,
    /// Float inputs are rounded to integer
    Integer,
    Float,
    ImageUrl,
    ImageBuffer,
    Choice(&'static [&'static str]),
    Optional(Box<CommandArgument>),
    OptionalWithDefault(Box<CommandArgumentType>, &'static str),
    OptionalWithDefaultDynamic(Box<CommandArgumentType>, fn())
}

/// Slash commands: These are treated as optional arguments (Equivalent to CommandArgumentType::Optional)
pub struct CommandFlag {
    pub name: String,
    pub description: String,
    pub r#type: CommandFlagType
}
pub enum CommandFlagType {
    /// Can be one word or "multiple words"
    String,
    /// Float inputs are rounded to integer
    Integer,
    Float,
    Choice(&'static [&'static str]),
    /// Slash commands: choice with "yes" or "no"
    Boolean
}

pub enum CommandAccess {
    Everyone,
    ServerManager,
    Private
}