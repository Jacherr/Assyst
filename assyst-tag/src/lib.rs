pub use context::Context;
pub use context::NopContext;
pub use parser::Parser;

mod context;
mod parser;
mod subtags;

pub fn parse<C: Context>(input: &str, args: &[&str], cx: C) -> Option<String> {
    Parser::new(input.as_bytes(), args, &cx).parse_segment()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let input = "{eval:{args}}";
        let segment = parse(input, &["{range:5|10}"], NopContext);
        println!("{:?}", segment);
    }
}
