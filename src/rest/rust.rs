use reqwest::{Client, Error};
use serde::Deserialize;
use serde_json::json;
use std::borrow::{Borrow, Cow};

const API_BASE: &str = "https://play.rust-lang.org";
const BENCHMARK_TEMPLATE: &str = r#"
#![feature(test)]
#[cfg(test)]
extern crate test;

use std::process::Command;

fn main() {
    let cmd = Command::new("cargo").arg("bench").output().unwrap();
    let cmd = String::from_utf8_lossy(if cmd.stdout.len() == 0 { &cmd.stderr } else { &cmd.stdout });
    
    println!("{}", cmd);
}

#[cfg(test)]
mod tests {
    {{code}}
}
"#;

#[derive(Deserialize)]
pub struct ApiResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

impl ApiResult {
    pub fn format(&self) -> &str {
        if self.stdout == "" {
            &self.stderr
        } else {
            &self.stdout
        }
    }
}

pub async fn run(
    client: &Client,
    code: &str,
    channel: Option<&str>,
    mode: Option<&str>,
    edition: Option<&str>,
    crate_type: Option<&str>,
    tests: Option<bool>,
) -> Result<ApiResult, Error> {
    client
        .post(&format!("{}/execute", API_BASE))
        .json(&json!({
            "code": code,
            "channel": channel.unwrap_or("stable"),
            "mode": mode.unwrap_or("debug"),
            "edition": edition.unwrap_or("2018"),
            "crateType": crate_type.unwrap_or("bin"),
            "tests": tests.unwrap_or(false)
        }))
        .send()
        .await?
        .json()
        .await
}

pub async fn run_binary(client: &Client, code: &str, channel: &str) -> Result<ApiResult, Error> {
    let code = if !code.contains("fn main") {
        Cow::Owned(format!("fn main() {{ println!(\"{{:?}}\", {{ {} }});}}", code))
    } else {
        Cow::Borrowed(code)
    };

    run(client, code.borrow(), Some(channel), None, None, None, None).await
}

pub async fn run_benchmark(client: &Client, code: &str) -> Result<ApiResult, Error> {
    let code = BENCHMARK_TEMPLATE.replace("{{code}}", code);

    run_binary(client, &code, "nightly").await
}
