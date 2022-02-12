use std::collections::HashMap;
use anyhow::{ensure, anyhow, Context as _};
use rand::prelude::ThreadRng;
use crate::{context::Context, subtags};

pub mod limits {
    pub const MAX_REQUESTS: u32 = 5;
    pub const MAX_VARIABLES: usize = 100;
    pub const MAX_VARIABLE_KEY_LENGTH: usize = 100;
    pub const MAX_VARIABLE_VALUE_LENGTH: usize = 5000;
    pub const MAX_ITERATIONS: u32 = 500;
    pub fn try_increment(field: &mut u32, limit: u32) -> bool {
        if *field >= limit {
            false
        } else {
            *field += 1;
            true
        }
    }
}

#[derive(Default)]
pub struct Counter {
    requests: u32,
    iterations: u32,
}

impl Counter {
    pub fn try_request(&mut self) -> bool {
        limits::try_increment(&mut self.requests, limits::MAX_REQUESTS)
    }
    pub fn try_iterate(&mut self) -> bool {
        limits::try_increment(&mut self.iterations, limits::MAX_ITERATIONS)
    }
}

pub struct Parser<'a> {
    input: &'a [u8],
    args: &'a [&'a str],
    idx: usize,
    rng: ThreadRng,
    variables: HashMap<Box<str>, Box<str>>,
    counter: Counter,
    cx: &'a dyn Context,
}

fn is_identifier(b: u8) -> bool {
    (b'a'..=b'z').contains(&b) || (b'A'..=b'Z').contains(&b)
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a [u8], args: &'a [&'a str], cx: &'a dyn Context) -> Self {
        Self {
            input,
            args,
            cx,
            idx: 0,
            rng: rand::thread_rng(),
            variables: HashMap::new(),
            counter: Counter::default(),
        }
    }

    pub fn read_identifier(&mut self) -> &'a [u8] {
        let start = self.idx;

        while self.idx < self.input.len() {
            let b = self.input[self.idx];

            if !is_identifier(b) {
                break;
            }

            self.idx += 1;
        }

        &self.input[start..self.idx]
    }

    pub fn parse_segment(&mut self) -> anyhow::Result<String> {
        ensure!(self.counter.try_iterate(), "Maximum number of iterations reached");

        let mut output = String::new();

        while self.idx < self.input.len() {
            let byte = self.input[self.idx];

            match byte {
                b'{' => {
                    // skip {
                    self.idx += 1;

                    // get subtag name, i.e. `range` in {range:1|10}
                    let name = std::str::from_utf8(self.read_identifier()).unwrap();

                    let mut args = Vec::new();

                    while let Some(b'|' | b':') = self.input.get(self.idx) {
                        // skip `|:`
                        self.idx += 1;

                        // recursively parse segment
                        args.push(self.parse_segment()?);
                    }

                    self.idx += 1;

                    let result = self.handle_tag(name, args).with_context(|| format!("An error occurred while processing {name}"))?;
                    output.push_str(&result);
                }
                b'|' | b'}' => {
                    break;
                }
                _ => {
                    output.push(byte as char);
                    self.idx += 1;
                }
            }
        }

        Ok(output)
    }

    pub fn handle_tag(&mut self, name: &str, args: Vec<String>) -> anyhow::Result<String> {
        match name {
            "repeat" => subtags::repeat(args),
            "range" => subtags::range(self, args),
            "eval" => subtags::eval(self, args),
            "arg" => subtags::arg(self, args),
            "args" => subtags::args(self),
            "set" => subtags::set(self, args),
            "get" => subtags::get(self, args),
            "delete" => subtags::delete(self, args),
            "argslen" => subtags::argslen(self),
            "abs" => subtags::abs(args),
            "cos" => subtags::cos(args),
            "sin" => subtags::sin(args),
            "tan" => subtags::tan(args),
            "sqrt" => subtags::sqrt(args),
            "e" => subtags::e(),
            "pi" => subtags::pi(),
            "max" => subtags::max(args),
            "min" => subtags::min(args),
            "choose" => subtags::choose(self, args),
            "length" => subtags::length(args),
            "lower" => subtags::lower(args),
            "upper" => subtags::upper(args),
            "replace" => subtags::replace(args),
            "reverse" => subtags::reverse(args),
            "js" | "javascript" => subtags::javascript(self, args),
            "lastattachment" => subtags::attachment_last(self),
            "avatar" => subtags::avatar(self, args),
            "download" => subtags::download(self, args),
            _ => Err(anyhow!("Unknown subtag: {name}")),
        }
    }

    pub fn args(&self) -> &[&str] {
        self.args
    }

    pub fn rng(&mut self) -> &mut ThreadRng {
        &mut self.rng
    }

    pub fn variables(&self) -> &HashMap<Box<str>, Box<str>> {
        &self.variables
    }

    pub fn variables_mut(&mut self) -> &mut HashMap<Box<str>, Box<str>> {
        &mut self.variables
    }

    pub fn context(&self) -> &dyn Context {
        &*self.cx
    }

    pub fn counter_mut(&mut self) -> &mut Counter {
        &mut self.counter
    }
}
