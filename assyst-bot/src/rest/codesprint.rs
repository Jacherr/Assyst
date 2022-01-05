use serde::{Deserialize, Serialize};

use crate::assyst::Assyst;

#[derive(Debug, Clone, Copy)]
pub enum Language {
    Rust,
}

#[derive(Serialize, Debug)]
pub struct Test {
    pub input: String,
    pub expect: String,
}

impl From<assyst_database::CodesprintTest> for Test {
    fn from(test: assyst_database::CodesprintTest) -> Self {
        Self {
            input: test.input,
            expect: test.expected,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct BenchmarkBody {
    code: String,
    tests: Vec<Test>,
}

#[derive(Deserialize, Debug)]
pub enum BenchmarkResponse {
    Success { mean: f64, iter: u64 },
    InvalidStatus { stderr: String },
    TestFail,
}

impl Language {
    /// Attempts to map a file extension string to a language.
    pub fn from_ext(s: &str) -> Option<Self> {
        match s {
            "rs" | "rust" => Some(Self::Rust),
            _ => None,
        }
    }

    /// Returns the language ID as it is stored in the database.
    pub fn to_database_id(&self) -> i16 {
        match self {
            Self::Rust => 1,
        }
    }
}

pub async fn benchmark(
    assyst: &Assyst,
    language: Language,
    code: &str,
    user_id: u64,
    tests: Vec<Test>,
) -> Result<BenchmarkResponse, reqwest::Error> {
    let url = assyst.config.url.codesprint.as_ref();
    let auth = assyst.config.auth.codesprint.as_ref();

    let url = format!("{}/bench", url);

    let re = assyst
        .reqwest_client
        .post(url)
        .json(&BenchmarkBody {
            code: code.to_string(),
            tests,
        })
        .header("Authorization", auth)
        .header("X-User-Id", &user_id.to_string())
        .send()
        .await?
        .json::<BenchmarkResponse>()
        .await?;

    Ok(re)
}
