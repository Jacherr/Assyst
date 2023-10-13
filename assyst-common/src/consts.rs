pub const WORKING_FILESIZE_LIMIT_BYTES: usize = 25_000_000;
pub const ABSOLUTE_INPUT_FILE_SIZE_LIMIT_BYTES: usize = 100_000_000;
pub const BOT_ID: usize = 526861829872549898; // if you change this, also update in ../config.toml
pub const MAX_CHAIN_LENGTH: usize = 100;
pub const BT_RATELIMIT_LEN: u64 = 2500;
pub const BT_RATELIMIT_MESSAGE: &str = "You are sending messages too quickly!";
pub const RANDOMIZE_COUNT: usize = 3;
pub const Y21: &'static str = "312715611413413889";
pub const MESSAGE_CHARACTER_LIMIT: usize = 2000;
pub const MESSAGE_EDIT_HANDLE_LIMIT: u32 = 60000;
pub const IDENTIFY_ERROR_MESSAGE: &str = "I really can't describe the picture :flushed:";
pub const MAX_TIMESTAMP: u64 = 8640000000000000;
pub const DEFAULT_COLORS: &[(&str, u32)] = &[
    ("gold", 0xf1c40f),
    ("teal", 0x1abc9c),
    ("darkpurple", 0x71368a),
    ("darkblue", 0x206694),
    ("salmon", 0xffa07a),
    ("lavender", 0xd1d1ff),
    ("lightred", 0xff4c4c),
    ("yellow", 0xfbf606),
    ("pink", 0xff69b4),
    ("lime", 0xff00),
    ("cyan", 0x8f8fc),
    ("white", 0xffffff),
    ("black", 0x10101),
    ("orange", 0xe67e22),
    ("blue", 0x3498db),
    ("purple", 0x8b00ff),
    ("green", 0x2ecc71),
    ("red", 0xe74c3c),
];
pub const EVENT_PIPE: &str = "/tmp/assyst-events.sock";
pub const CACHE_PIPE: &str = "/tmp/assyst-cache.sock";
pub mod gateway {
    use std::collections::HashMap;

    use serde::{Serialize, Deserialize};

    pub const OP_EVENT: u8 = 0;
    pub const OP_LATENCIES: u8 = 1;
    #[derive(Serialize, Deserialize)]
    pub struct Latencies(pub HashMap<u64, i64>);
}
pub const CANNOT_REPLY_WITHOUT_MESSAGE_HISTORY_CODE: u64 = 160002;