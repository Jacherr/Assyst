use std::time::{SystemTime, UNIX_EPOCH};

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
