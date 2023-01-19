use crate::filetype;
use bytes::Bytes;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FakeEvalMessageData<M: Serialize> {
    pub message: M
}

#[derive(Serialize)]
pub struct FakeEvalBody<M: Serialize> {
    pub code: String,
    pub data: Option<FakeEvalMessageData<M>>
}

#[derive(Deserialize)]
pub struct FakeEvalResponse {
    pub message: String,
}

pub enum FakeEvalImageResponse {
    Text(FakeEvalResponse),
    Image(Bytes, filetype::Type),
}
