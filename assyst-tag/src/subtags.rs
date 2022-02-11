use rand::Rng;

use crate::parser::{limits, Parser};

macro_rules! ensure {
    ($x:expr) => {
        if !$x {
            return None;
        }
    };
}

pub fn repeat(args: Vec<String>) -> Option<String> {
    let count = args.get(0)?.parse::<usize>().ok()?;
    let input = args.get(1)?;
    Some(input.repeat(count))
}

pub fn range(parser: &mut Parser, args: Vec<String>) -> Option<String> {
    let lower = args.get(0)?.parse::<usize>().ok()?;
    let upper = args.get(1)?.parse::<usize>().ok()?;
    let out = parser.rng().gen_range(lower..upper);
    Some(out.to_string())
}

pub fn eval(parser: &mut Parser, args: Vec<String>) -> Option<String> {
    ensure!(parser.counter_mut().try_request());

    let text = args.get(0)?;
    crate::parse(text, parser.args(), parser.context())
}

pub fn arg(parser: &Parser) -> Option<String> {
    parser.args().get(0).map(|&s| s.to_owned())
}

pub fn args(parser: &Parser) -> Option<String> {
    let args = parser.args();
    Some(args.join(" "))
}

pub fn set(parser: &mut Parser, args: Vec<String>) -> Option<String> {
    let mut iter = args.into_iter();
    let key = iter.next()?;
    let value = iter.next()?;

    let variables = parser.variables_mut();
    ensure!(variables.len() < limits::MAX_VARIABLES);
    ensure!(key.len() < limits::MAX_VARIABLE_KEY_LENGTH);
    ensure!(value.len() < limits::MAX_VARIABLE_VALUE_LENGTH);

    variables.insert(key.into(), value.into());
    Some(String::new())
}

pub fn get(parser: &Parser, args: Vec<String>) -> Option<String> {
    let key = args.get(0)?;
    parser
        .variables()
        .get(key.as_str())
        .map(|s| s.as_ref().to_owned())
}

pub fn delete(parser: &mut Parser, args: Vec<String>) -> Option<String> {
    let key = args.get(0)?;
    parser.variables_mut().remove(key.as_str());
    Some(String::new())
}

pub fn argslen(parser: &Parser) -> Option<String> {
    Some(parser.args().len().to_string())
}

pub fn abs(args: Vec<String>) -> Option<String> {
    Some(i32::abs(args.get(0)?.parse().ok()?).to_string())
}

pub fn cos(args: Vec<String>) -> Option<String> {
    Some(f32::cos(args.get(0)?.parse().ok()?).to_string())
}

pub fn sin(args: Vec<String>) -> Option<String> {
    Some(f32::sin(args.get(0)?.parse().ok()?).to_string())
}

pub fn tan(args: Vec<String>) -> Option<String> {
    Some(f32::tan(args.get(0)?.parse().ok()?).to_string())
}

pub fn sqrt(args: Vec<String>) -> Option<String> {
    Some(f32::sqrt(args.get(0)?.parse().ok()?).to_string())
}

pub fn e() -> Option<String> {
    Some(std::f64::EPSILON.to_string())
}

pub fn pi() -> Option<String> {
    Some(std::f64::consts::PI.to_string())
}

pub fn max(args: Vec<String>) -> Option<String> {
    args.into_iter()
        .flat_map(|s| s.parse::<i32>().ok())
        .max()
        .map(|n| n.to_string())
}

pub fn min(args: Vec<String>) -> Option<String> {
    args.into_iter()
        .flat_map(|s| s.parse::<i32>().ok())
        .max()
        .map(|n| n.to_string())
}

pub fn choose(parser: &mut Parser, args: Vec<String>) -> Option<String> {
    let idx = parser.rng().gen_range(0..args.len());
    args.get(idx).cloned()
}

pub fn length(args: Vec<String>) -> Option<String> {
    let text = args.get(0)?;
    Some(text.len().to_string())
}

pub fn lower(args: Vec<String>) -> Option<String> {
    let mut text = args.into_iter().next()?;
    text.make_ascii_lowercase();
    Some(text)
}

pub fn upper(args: Vec<String>) -> Option<String> {
    let mut text = args.into_iter().next()?;
    text.make_ascii_uppercase();
    Some(text)
}

pub fn replace(args: Vec<String>) -> Option<String> {
    let mut iter = args.into_iter();
    let what = iter.next()?;
    let with = iter.next()?;
    let text = iter.next()?;

    Some(text.replace(&what, &with))
}

pub fn reverse(args: Vec<String>) -> Option<String> {
    let mut text = args.get(0)?.as_bytes().to_owned();
    text.reverse();
    Some(String::from_utf8_lossy(&text).into_owned())
}

pub fn javascript(parser: &mut Parser, args: Vec<String>) -> Option<String> {
    ensure!(parser.counter_mut().try_request());

    let code = args.get(0)?;
    parser.context().execute_javascript(code)
}

pub fn attachment_last(parser: &mut Parser) -> Option<String> {
    ensure!(parser.counter_mut().try_request());

    parser.context().get_last_attachment()
}

pub fn avatar(parser: &mut Parser, args: Vec<String>) -> Option<String> {
    ensure!(parser.counter_mut().try_request());

    let user_id = args.first().map(|s| s.parse::<u64>().ok()).flatten();
    parser.context().get_avatar(user_id)
}

pub fn download(parser: &mut Parser, args: Vec<String>) -> Option<String> {
    ensure!(parser.counter_mut().try_request());

    let url = args.get(0)?;
    parser.context().download(url)
}
