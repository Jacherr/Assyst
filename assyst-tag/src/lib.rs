use assyst_common::filetype::Type;
use bytes::Bytes;
pub use context::Context;
pub use context::NopContext;
use parser::Counter;
pub use parser::Parser;
use parser::SharedState;
use std::cell::RefCell;
use std::collections::HashMap;

mod context;
mod parser;
mod subtags;

#[derive(Debug)]
pub struct ParseResult {
    pub output: String,
    pub attachment: Option<(Bytes, Type)>,
}

pub fn parse<C: Context>(input: &str, args: &[&str], cx: C) -> anyhow::Result<ParseResult> {
    let variables = RefCell::new(HashMap::new());
    let counter = Counter::default();
    let attachment = RefCell::new(None);
    let state = SharedState::new(&variables, &counter, &attachment);

    let output = Parser::new(input.as_bytes(), args, state, &cx).parse_segment(true)?;

    Ok(ParseResult {
        output,
        attachment: attachment.into_inner(),
    })
}

pub fn parse_with_parent(
    input: &str,
    parent: &Parser,
    side_effects: bool,
) -> anyhow::Result<String> {
    Parser::from_parent(input.as_bytes(), parent).parse_segment(side_effects)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let input = "testing \\{ abc";
        let segment = parse(input, &["h", "o"], NopContext);
        match segment {
            Ok(r) => println!("{r:?}"),
            Err(e) => println!("Error: {:?}", e),
        }
    }

    #[test]
    fn tags_invoke_each_other() {
        let input = "tag content: {tag:wtf|a|b}!";
        let segment = parse(input, &["h", "o"], NopContext);
        match segment {
            Ok(r) => println!("{r:?}"),
            Err(e) => println!("Error: {:?}", e),
        }
    }
}
