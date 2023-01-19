use std::time::{SystemTime, UNIX_EPOCH};

use twilight_model::id::{marker::{ChannelMarker, GuildMarker, UserMarker, WebhookMarker, MessageMarker, RoleMarker}, Id};

pub type ChannelId = Id<ChannelMarker>;
pub type GuildId = Id<GuildMarker>;
pub type UserId = Id<UserMarker>;
pub type WebhookId = Id<WebhookMarker>;
pub type MessageId = Id<MessageMarker>;
pub type RoleId = Id<RoleMarker>;

/// Returns the current timestamp in milliseconds
pub fn get_current_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
        .try_into()
        .expect("Couldn't fit timestamp (u128) into i64")
}

/// Promotes the lifetime of a string to a static string by leaking memory
pub fn to_static_str(s: &Box<str>) -> &'static mut str {
    Box::leak(s.clone())
}

#[macro_export]
macro_rules! ok_or_break {
    ($expression:expr) => {
        match $expression {
            Ok(v) => v,
            Err(_) => break,
        }
    };
}

#[macro_export]
macro_rules! some_or_break {
    ($expression:expr) => {
        match $expression {
            Some(v) => v,
            None => break,
        }
    };
}

#[macro_export]
macro_rules! unwrap_enum_variant {
    ($expression:expr, $variant:path) => {
        match $expression {
            $variant (v) => v,
            _ => unreachable!(),
        }
    };
}

pub mod regexes {
    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        pub static ref MENTION: Regex = Regex::new(r"(?:<@!?)?(\d{16,20})>?").unwrap();
    }
}

pub fn mention_to_id(s: &str) -> Option<u64> {
    regexes::MENTION
        .captures(s)
        .and_then(|capture| capture.get(1))
        .and_then(|id| Some(id.as_str()))
        .and_then(|id| id.parse::<u64>().ok())
}
