use serde::{Deserialize, Serialize};

use crate::assyst::Assyst;

use super::routes;

#[derive(Debug, Clone, Copy)]
pub enum Language {
    Rust,
    JavaScript,
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
            "js" | "javascript" => Some(Self::JavaScript),
            _ => None,
        }
    }

    /// Returns the language ID as it is stored in the database.
    pub fn to_database_id(&self) -> i16 {
        match self {
            Self::Rust => 1,
            Self::JavaScript => 2,
        }
    }

    pub fn to_request_pair<'a>(&self, assyst: &'a Assyst) -> (String, Option<&'a str>) {
        match self {
            Self::Rust => (
                format!("{}/bench", assyst.config.url.codesprint.as_ref()),
                Some(assyst.config.auth.codesprint.as_ref()),
            ),
            Self::JavaScript => (format!("{}?bench=true", routes::FAKE_EVAL), None),
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
    let (url, auth) = language.to_request_pair(assyst);

    let mut re = assyst.reqwest_client.post(url).json(&BenchmarkBody {
        code: code.to_string(),
        tests,
    });

    if let Some(auth) = auth {
        re = re.header("Authorization", auth);
    }

    let re = re
        .header("X-User-Id", &user_id.to_string())
        .send()
        .await?
        .json::<BenchmarkResponse>()
        .await?;

    Ok(re)
}
