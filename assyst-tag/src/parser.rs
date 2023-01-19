use crate::{context::Context, subtags};
use anyhow::{anyhow, ensure, Context as _};
use assyst_common::filetype::Type;
use bytes::Bytes;
use rand::prelude::ThreadRng;
use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
};

/// Constants and helper functions for tag parser limits
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

/// Parser state that is shared across multiple parsers
///
/// See comment in `Parser::from_parent` for more details.
#[derive(Clone)]
pub struct SharedState<'a> {
    /// User defined variables
    variables: &'a RefCell<HashMap<String, String>>,
    /// Counter for various limits
    counter: &'a Counter,
    /// The attachment to be responded with, if set
    attachment: &'a RefCell<Option<(Bytes, Type)>>,
}

impl<'a> SharedState<'a> {
    pub fn new(
        variables: &'a RefCell<HashMap<String, String>>,
        counter: &'a Counter,
        attachment: &'a RefCell<Option<(Bytes, Type)>>,
    ) -> Self {
        Self {
            variables,
            counter,
            attachment,
        }
    }

    /// Calls `f` with a mutable reference to the user defined variables
    pub fn with_variables_mut<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&mut HashMap<String, String>) -> T,
    {
        let mut variables = self.variables.borrow_mut();
        f(&mut *variables)
    }

    /// Calls `f` with a reference to the user defined variables
    pub fn with_variables<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&HashMap<String, String>) -> T,
    {
        let variables = self.variables.borrow();
        f(&*variables)
    }

    /// Returns a reference to the counter
    pub fn counter(&self) -> &Counter {
        &self.counter
    }

    /// Sets the attachment to be responded with
    pub fn set_attachment(&self, buf: Bytes, ty: Type) {
        *self.attachment.borrow_mut() = Some((buf, ty));
    }
}

/// Counter for various limits
#[derive(Default)]
pub struct Counter {
    /// Number of HTTP requests
    requests: Cell<u32>,
    /// Number of parser iterations
    iterations: Cell<u32>,
}

impl Counter {
    /// Tries to increment the requests field if it's not already at the limit
    pub fn try_request(&self) -> bool {
        limits::try_increment(&self.requests, limits::MAX_REQUESTS)
    }

    /// Tries to increment the iterations field if it's not already at the limit
    pub fn try_iterate(&self) -> bool {
        limits::try_increment(&self.iterations, limits::MAX_ITERATIONS)
    }
}

/// The tag parser
pub struct Parser<'a> {
    /// The input string
    input: &'a [u8],
    /// Tag arguments, accessible through {arg} and {args}
    args: &'a [&'a str],
    /// Current index in the input string
    idx: usize,
    /// Shared parser state across multiple parsers
    state: SharedState<'a>,
    /// A cached `ThreadRng`, used to generate random numbers
    rng: ThreadRng,
    /// Context for this parser
    cx: &'a dyn Context,
    /// Recursive depth, to avoid stack overflow in {eval} calls
    depth: u32,
}

/// Checks if a given byte is in the a..z A..Z range
fn is_identifier(b: u8) -> bool {
    (b'a'..=b'z').contains(&b) || (b'A'..=b'Z').contains(&b)
}

impl<'a> Parser<'a> {
    /// Creates a parser with shared state from the parent parser
    ///
    /// The returned parser shares the same limits and variables with `other`
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

    /// Creates a new parser
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

    /// Eats a byte
    pub fn eat(&mut self, bs: &[u8]) -> bool {
        if let Some(b) = self.input.get(self.idx) {
            if bs.contains(b) {
                self.idx += 1;
                return true;
            }
        }
        false
    }

    /// Eats a separator
    pub fn eat_separator(&mut self) -> bool {
        self.eat(b":|")
    }

    /// Checks if the current character is escaped, i.e. the last character is \
    pub fn is_escaped(&self) -> bool {
        self.input.get(self.idx - 1) == Some(&b'\\')
    }

    /// Parses a single "segment" of the input
    ///
    /// A segment can be a single argument of a tag, or the entire input itself.
    /// Sometimes it's necessary to parse without "side effects", which means
    /// that it needs to parse a segment without actually invoking the tag handler.
    ///
    /// For example, given `a{note:{arg:this_would_error}}b`, if we want to skip the note tag
    /// such that we end up with `ab`, we need to parse it without calling the `arg` tag handler.
    /// If we *did* invoke it, this would return an error
    pub fn parse_segment(&mut self, side_effects: bool) -> anyhow::Result<String> {
        ensure!(
            self.state.counter.try_iterate(),
            "Maximum number of iterations reached"
        );

        if !side_effects {
            // If this call isn't allowed to have any side effects, we can just "fast-forward" to the next }
            // that matches the depth of the current call.
            let mut depth = 1;
            let mut bytes = Vec::new();

            while self.idx < self.input.len() {
                let byte = self.input[self.idx];

                match byte {
                    b'{' if !self.is_escaped() => depth += 1,
                    b'}' if !self.is_escaped() => {
                        depth -= 1;
                        if depth == 0 {
                            break;
                        }
                    }
                    b'|' if !self.is_escaped() && depth <= 1 => break,
                    _ => {}
                }

                bytes.push(byte);
                self.idx += 1;
            }
            return Ok(String::from_utf8_lossy(&bytes).to_string());
        }

        let mut output = String::new();

        while self.idx < self.input.len() {
            let byte = self.input[self.idx];

            match byte {
                b'{' => {
                    // skip {
                    self.idx += 1;

                    // get subtag name, i.e. `range` in {range:1|10}
                    let name = std::str::from_utf8(self.read_identifier()).unwrap();

                    // lazy tags need to be evaluated before the args are parsed
                    // see comment in `handle_lazy_tag` for what it means for a tag to be lazy
                    if let Some(re) = self.handle_lazy_tag(name) {
                        output.push_str(&re?);
                        continue;
                    }

                    let mut args = Vec::new();

                    while let Some(b'|' | b':') = self.input.get(self.idx) {
                        // skip `|:`
                        self.idx += 1;

                        // recursively parse segment
                        args.push(self.parse_segment(side_effects)?);
                    }

                    self.idx += 1;

                    let result = if side_effects {
                        self.handle_tag(name, args)
                            .with_context(|| format!("An error occurred while processing {name}"))?
                    } else {
                        String::new()
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
                    // If we are escaping | or }, then only push *that* character, and not \
                    if byte == b'\\' {
                        if let Some(&next @ b'|' | &next @ b'}' | &next @ b'{') =
                            self.input.get(self.idx + 1)
                        {
                            output.push(next as char);
                            self.idx += 2;
                            continue;
                        }
                    }

                    output.push(byte as char);
                    self.idx += 1;
                }
            }
        }

        Ok(output)
    }

    /// Handles a "lazy" tag
    ///
    /// Lazy tags are subtags whose arguments are parsed by the subtag itself, and not by the parser beforehand.
    /// This is needed for special subtags like if, which needs to decide whether to parse `then` or else`
    /// only after it compared two arguments
    pub fn handle_lazy_tag(&mut self, name: &str) -> Option<anyhow::Result<String>> {
        match name {
            "if" => Some(subtags::r#if(self)),
            "note" => Some(subtags::note(self)),
            "ignore" => Some(subtags::ignore(self)),
            _ => None,
        }
    }

    /// Handles a regular tag
    pub fn handle_tag(&mut self, name: &str, args: Vec<String>) -> anyhow::Result<String> {
        match name {
            "repeat" => subtags::repeat(args),
            "range" => subtags::range(self, args),
            "eval" => subtags::eval(self, args),
            "tryarg" => subtags::tryarg(self, args),
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
            "channelid" => subtags::channelid(self),
            "usertag" => subtags::usertag(self, args),
            "js" | "javascript" => subtags::javascript(self, args),
            "lastattachment" => subtags::attachment_last(self),
            "avatar" => subtags::avatar(self, args),
            "download" => subtags::download(self, args),
            "mention" => subtags::mention(self, args),
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
