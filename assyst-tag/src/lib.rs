use std::cell::RefCell;
use std::collections::HashMap;

pub use context::Context;
pub use context::NopContext;
use parser::Counter;
pub use parser::Parser;
use parser::SharedState;

mod context;
mod parser;
mod subtags;

pub fn parse<C: Context>(input: &str, args: &[&str], cx: C) -> anyhow::Result<String> {
    let variables = RefCell::new(HashMap::new());
    let counter = Counter::default();
    let state = SharedState::new(&variables, &counter);

    Parser::new(input.as_bytes(), args, state, &cx).parse_segment(true)
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
        let input = "a{if:abc|=|abc|c{note:ignore me}d|{arg:1}}b";
        let segment = parse(input, &["h", "o"], NopContext);
        match segment {
            Ok(r) => println!("{r}"),
            Err(e) => println!("Error: {:?}", e),
        }
    }
}
