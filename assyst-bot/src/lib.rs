#![allow(dead_code, unused)]

#[macro_use]
extern crate dlopen_derive;

pub mod ansi;
pub mod assyst;
pub mod badtranslator;
pub mod caching;
pub mod command;
pub mod downloader;
pub mod eval;
pub mod handler;
pub mod handlers;
pub mod logger;
pub mod metrics;
pub mod rest;
pub mod tasks;
pub mod util;

use crate::assyst::Assyst;
use assyst_common::consts::{
    gateway::{self, Latencies},
    EVENT_PIPE,
};
use assyst_webserver::run as webserver_run;
use bincode::deserialize;
use caching::persistent_caching::get_guild_count;
use handler::handle_event;
use serde::de::DeserializeSeed;
use std::sync::Arc;
use tokio::{
    io::{AsyncReadExt, BufReader},
    net::UnixStream,
};
use twilight_model::gateway::event::GatewayEventDeserializer;
