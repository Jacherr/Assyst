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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let input = "{repeat:10}";
        let segment = parse(input, &["{range:5|10}"], NopContext);
        match segment {
            Ok(r) => println!("{r}"),
            Err(e) => println!("Error: {:?}", e),
        }
    }
}
