use crate::parser::{
    limits::{self, MAX_DEPTH, MAX_STRING_LENGTH},
    Parser,
};
use anyhow::Context;
use rand::Rng;

use anyhow::ensure;

/// Ensures that the HTTP request limit has not been hit yet
///
/// This should be called in tags that issue HTTP requests in any way.
/// Returns with an error if the limit is reached
macro_rules! ensure_request_limit {
    ($parser:expr) => {
        ensure!(
            $parser.state().counter().try_request(),
            "Maximum number of HTTP requests reached"
        );
    };
}

#[rustfmt::skip]
pub fn repeat(args: Vec<String>) -> anyhow::Result<String> {
    let count = args.first().context("Missing count argument")?.parse()?;
    let input = args.get(1).context("Missing input agument")?;

    ensure!(input.len() + count < MAX_STRING_LENGTH, "String exceeds maximum length of {MAX_STRING_LENGTH} bytes");

    Ok(input.repeat(count))
}

pub fn range(parser: &mut Parser, args: Vec<String>) -> anyhow::Result<String> {
    let lower = args.first().context("Missing lower bound")?.parse()?;
    let upper = args.get(1).context("Missing upper bound")?.parse()?;
    let out: usize = parser.rng().gen_range(lower..=upper);

    Ok(out.to_string())
}

#[rustfmt::skip]
pub fn eval(parser: &mut Parser, args: Vec<String>) -> anyhow::Result<String> {
    let text = args.first().context("Missing input argument")?;

    ensure!(parser.depth() < MAX_DEPTH, "Maximum recursion depth reached ({MAX_DEPTH})");

    Parser::from_parent(text.as_bytes(), parser).parse_segment(true)
}

pub fn arg(parser: &Parser, args: Vec<String>) -> anyhow::Result<String> {
    let arg = args
        .first()
        .context("Missing index argument")?
        .parse::<usize>()?;

    let arg = parser
        .args()
        .get(arg)
        .with_context(|| format!("Index {arg} is out of bounds"))?;

    Ok(arg.to_string())
}

pub fn args(parser: &Parser) -> anyhow::Result<String> {
    let args = parser.args();
    Ok(args.join(" "))
}

#[rustfmt::skip]
pub fn set(parser: &mut Parser, args: Vec<String>) -> anyhow::Result<String> {
    let mut iter = args.into_iter();
    let key = iter.next().context("Missing key argument")?;
    let value = iter.next().context("Missing value argument")?;

    parser.state().with_variables_mut(move |variables| -> anyhow::Result<String> {
        ensure!(variables.len() < limits::MAX_VARIABLES, "Maximum number of variables reached");
        ensure!(key.len() < limits::MAX_VARIABLE_KEY_LENGTH, "Key exceeds maximum length of {}", limits::MAX_VARIABLE_KEY_LENGTH);
        ensure!(value.len() < limits::MAX_VARIABLE_VALUE_LENGTH, "Value exceeds maximum length of {}", limits::MAX_VARIABLE_VALUE_LENGTH);
        
        variables.insert(key.into(), value.into());
        Ok(String::new())
    })
}

pub fn get(parser: &Parser, args: Vec<String>) -> anyhow::Result<String> {
    let key = args.first().context("Missing key argument")?;

    parser.state().with_variables(|variables| {
        Ok(variables
            .get(key.as_str())
            .map(|s| s.clone())
            .unwrap_or_else(String::new))
    })
}

pub fn delete(parser: &mut Parser, args: Vec<String>) -> anyhow::Result<String> {
    let key = args.first().context("Missing key argument")?;

    parser
        .state()
        .with_variables_mut(|variables| variables.remove(key.as_str()));

    Ok(String::new())
}

pub fn argslen(parser: &Parser) -> anyhow::Result<String> {
    Ok(parser.args().len().to_string())
}

pub fn abs(args: Vec<String>) -> anyhow::Result<String> {
    let arg = args.first().context("Missing input argument")?.parse()?;
    Ok(i32::abs(arg).to_string())
}

pub fn cos(args: Vec<String>) -> anyhow::Result<String> {
    let arg = args.first().context("Missing input argument")?.parse()?;
    Ok(f32::cos(arg).to_string())
}

pub fn sin(args: Vec<String>) -> anyhow::Result<String> {
    let arg = args.first().context("Missing input argument")?.parse()?;
    Ok(f32::sin(arg).to_string())
}

pub fn tan(args: Vec<String>) -> anyhow::Result<String> {
    let arg = args.first().context("Missing input argument")?.parse()?;
    Ok(f32::tan(arg).to_string())
}

pub fn sqrt(args: Vec<String>) -> anyhow::Result<String> {
    let arg = args.first().context("Missing input argument")?.parse()?;
    Ok(f32::sqrt(arg).to_string())
}

pub fn e() -> anyhow::Result<String> {
    Ok(std::f64::EPSILON.to_string())
}

pub fn pi() -> anyhow::Result<String> {
    Ok(std::f64::consts::PI.to_string())
}

pub fn max(args: Vec<String>) -> anyhow::Result<String> {
    let mut max = args.first().context("No arguments present")?.parse()?;

    for arg in args.iter().skip(1) {
        let arg = arg.parse()?;

        if arg > max {
            max = arg;
        }
    }

    Ok(i32::to_string(&max))
}

pub fn min(args: Vec<String>) -> anyhow::Result<String> {
    let mut min = args.first().context("No arguments present")?.parse()?;

    for arg in args.iter().skip(1) {
        let arg = arg.parse()?;

        if arg < min {
            min = arg;
        }
    }

    Ok(i32::to_string(&min))
}

pub fn choose(parser: &mut Parser, args: Vec<String>) -> anyhow::Result<String> {
    let idx = parser.rng().gen_range(0..args.len());
    // Generated index is always in bounds, except when args.is_empty()
    // So if this returns None, it means that there are no arguments
    args.get(idx).cloned().context("No arguments present")
}

pub fn length(args: Vec<String>) -> anyhow::Result<String> {
    let text = args.first().context("Missing text argument")?;
    Ok(text.len().to_string())
}

pub fn lower(args: Vec<String>) -> anyhow::Result<String> {
    let mut text = args.into_iter().next().context("Missing text argument")?;
    text.make_ascii_lowercase();
    Ok(text)
}

pub fn upper(args: Vec<String>) -> anyhow::Result<String> {
    let mut text = args.into_iter().next().context("Missing text argument")?;
    text.make_ascii_uppercase();
    Ok(text)
}

pub fn replace(args: Vec<String>) -> anyhow::Result<String> {
    let mut iter = args.into_iter();
    let what = iter.next().context("Missing text to be replaced")?;
    let with = iter.next().context("Missing replacer")?;
    let text = iter.next().context("Missing source argument")?;

    Ok(text.replace(&what, &with))
}

pub fn reverse(args: Vec<String>) -> anyhow::Result<String> {
    let text = args.first().context("No text argument")?;
    let mut text = text.as_bytes().to_owned();
    text.reverse();
    Ok(String::from_utf8_lossy(&text).into_owned())
}

pub fn javascript(parser: &mut Parser, args: Vec<String>) -> anyhow::Result<String> {
    ensure_request_limit!(parser);

    let code = args.first().context("Missing code argument")?;
    parser.context().execute_javascript(code)
}

pub fn attachment_last(parser: &mut Parser) -> anyhow::Result<String> {
    ensure_request_limit!(parser);

    parser.context().get_last_attachment()
}

pub fn avatar(parser: &mut Parser, args: Vec<String>) -> anyhow::Result<String> {
    ensure_request_limit!(parser);

    let user_id = args.first().map(|s| s.parse()).transpose()?;
    parser.context().get_avatar(user_id)
}

pub fn download(parser: &mut Parser, args: Vec<String>) -> anyhow::Result<String> {
    ensure_request_limit!(parser);

    let url = args.first().context("Missing URL argument")?;
    parser.context().download(url)
}
