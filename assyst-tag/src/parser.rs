use crate::{context::Context, subtags};
use anyhow::{anyhow, ensure, Context as _};
use rand::prelude::ThreadRng;
use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
};

pub mod limits {
    use std::cell::Cell;

    pub const MAX_REQUESTS: u32 = 5;
    pub const MAX_VARIABLES: usize = 100;
    pub const MAX_VARIABLE_KEY_LENGTH: usize = 100;
    pub const MAX_VARIABLE_VALUE_LENGTH: usize = MAX_STRING_LENGTH;
    pub const MAX_ITERATIONS: u32 = 500;
    pub const MAX_DEPTH: u32 = 15;
    pub const MAX_STRING_LENGTH: usize = 10000;

    pub fn try_increment(field_cell: &Cell<u32>, limit: u32) -> bool {
        let field = field_cell.get();
        if field >= limit {
            false
        } else {
            field_cell.set(field + 1);
            true
        }
    }
}

#[derive(Clone)]
pub struct SharedState<'a> {
    variables: &'a RefCell<HashMap<String, String>>,
    counter: &'a Counter,
}

impl<'a> SharedState<'a> {
    pub fn new(variables: &'a RefCell<HashMap<String, String>>, counter: &'a Counter) -> Self {
        Self { variables, counter }
    }

    pub fn with_variables_mut<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&mut HashMap<String, String>) -> T,
    {
        let mut variables = self.variables.borrow_mut();
        f(&mut *variables)
    }

    pub fn with_variables<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&HashMap<String, String>) -> T,
    {
        let variables = self.variables.borrow();
        f(&*variables)
    }

    pub fn counter(&self) -> &Counter {
        &self.counter
    }
}

#[derive(Default)]
pub struct Counter {
    requests: Cell<u32>,
    iterations: Cell<u32>,
}

impl Counter {
    pub fn try_request(&self) -> bool {
        limits::try_increment(&self.requests, limits::MAX_REQUESTS)
    }
    pub fn try_iterate(&self) -> bool {
        limits::try_increment(&self.iterations, limits::MAX_ITERATIONS)
    }
}

pub struct Parser<'a> {
    input: &'a [u8],
    args: &'a [&'a str],
    idx: usize,
    state: SharedState<'a>,
    rng: ThreadRng,
    cx: &'a dyn Context,
    depth: u32,
}

fn is_identifier(b: u8) -> bool {
    (b'a'..=b'z').contains(&b) || (b'A'..=b'Z').contains(&b)
}

impl<'a> Parser<'a> {
    pub fn from_parent(input: &'a [u8], other: &Self) -> Self {
        Self {
            input,
            args: other.args,
            idx: 0,
            state: other.state.clone(),
            rng: rand::thread_rng(),
            cx: other.cx,
            depth: other.depth + 1,
        }
    }

    pub fn new(
        input: &'a [u8],
        args: &'a [&'a str],
        state: SharedState<'a>,
        cx: &'a dyn Context,
    ) -> Self {
        Self {
            input,
            args,
            cx,
            idx: 0,
            state,
            rng: rand::thread_rng(),
            depth: 0,
        }
    }

    /// Reads bytes from input until the first non-identifier byte is found, increasing the internal index on
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

    pub fn parse_segment(&mut self, mut side_effects: bool) -> anyhow::Result<String> {
        ensure!(
            self.state.counter.try_iterate(),
            "Maximum number of iterations reached"
        );

        let mut output = String::new();

        while self.idx < self.input.len() {
            let byte = self.input[self.idx];

            match byte {
                b'{' => {
                    // skip {
                    self.idx += 1;

                    // get subtag name, i.e. `range` in {range:1|10}
                    let name = std::str::from_utf8(self.read_identifier()).unwrap();

                    // check if tag is allowed to have "side effects"
                    side_effects &= !matches!(name, "note" | "ignore");

                    let mut args = Vec::new();

                    while let Some(b'|' | b':') = self.input.get(self.idx) {
                        // skip `|:`
                        self.idx += 1;

                        // recursively parse segment
                        args.push(self.parse_segment(side_effects)?);
                    }

                    self.idx += 1;

                    // handle special cased tags
                    let result = match name {
                        "note" => String::new(),
                        "ignore" => args.into_iter().next().unwrap_or_else(String::new),
                        _ => {
                            // only actually evaluate tag if it's allowed to have side effects
                            if side_effects {
                                self.handle_tag(name, args).with_context(|| {
                                    format!("An error occurred while processing {name}")
                                })?
                            } else {
                                String::new()
                            }
                        }
                    };

                    ensure!(
                        output.len() + result.len() < limits::MAX_STRING_LENGTH,
                        "Output string exceeds maximum string length of {} bytes",
                        limits::MAX_STRING_LENGTH
                    );

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

    pub fn state(&self) -> &SharedState<'a> {
        &self.state
    }

    pub fn depth(&self) -> u32 {
        self.depth
    }

    pub fn context(&self) -> &dyn Context {
        &*self.cx
    }
}
