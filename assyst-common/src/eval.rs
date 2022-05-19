use crate::filetype;
use bytes::Bytes;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct FakeEvalBody {
    pub code: String,
}

#[derive(Deserialize)]
pub struct FakeEvalResponse {
    pub message: String,
}

pub enum FakeEvalImageResponse {
    Text(FakeEvalResponse),
    Image(Bytes, filetype::Type),
}
