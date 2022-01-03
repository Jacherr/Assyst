use std::sync::Arc;

pub mod bt;
pub mod cache;
pub mod config;
pub mod consts;
pub mod util;

pub type HttpClient = Arc<twilight_http::Client>;