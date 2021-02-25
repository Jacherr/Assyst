use bytes::Bytes;
use futures_util::StreamExt;
use std::{borrow::Cow, convert::TryInto, num::ParseIntError, time::{SystemTime, UNIX_EPOCH}};

#[macro_export]
macro_rules! box_str {
    ($str:expr) => {
        $str.to_owned().into_boxed_str()
    };
}

pub mod regexes {
    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        pub static ref CUSTOM_EMOJI: Regex = Regex::new(r"<a?:\w+:(\d{16,20})>").unwrap();
        pub static ref TENOR_GIF: Regex = Regex::new(r"https://media1\.tenor\.com/images/[a-zA-Z0-9]+/tenor\.gif").unwrap();
        pub static ref URL: Regex = Regex::new(r"https?://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)").unwrap();
        pub static ref USER_MENTION: Regex = Regex::new(r"(?:<@!?)?(\d{16,20})>?").unwrap();
        pub static ref TIME_STRING: Regex = Regex::new("(\\d+)([smhd])").unwrap();
    }
}

mod file_signatures {
    pub const GIF: [u8; 3] = [71, 73, 70];
    pub const JPEG: [u8; 3] = [255, 216, 255];
    pub const PNG: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
}

pub fn get_buffer_filetype(buffer: &Bytes) -> Option<&'static str> {
    let first_3_bytes = buffer.iter().take(3);
    if first_3_bytes.clone().eq(&file_signatures::GIF) {
        Some("gif")
    } else if first_3_bytes.eq(&file_signatures::JPEG) {
        Some("jpeg")
    } else {
        let first_8_bytes = buffer.iter().take(8);
        if first_8_bytes.eq(&file_signatures::PNG) {
            Some("png")
        } else {
            None
        }
    }
}

pub fn get_current_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
        .try_into()
        .expect("Couldn't fit timestamp (u128) into i64")
}

pub fn get_longer_str<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

pub fn generate_table(input: &[(&str, &str)]) -> String {
    let longest = input.iter().fold(input[0].0, |previous, (current, _)| {
        get_longer_str(previous, current)
    });

    input
        .iter()
        .map(|(key, value)| {
            format!(
                "{}{}: {}\n",
                " ".repeat(longest.len() - key.len()),
                key,
                value
            )
        })
        .fold(String::new(), |a, b| a + &b)
}

pub fn codeblock(code: &str, language: &str) -> String {
    format!(
        "```{}\n{}\n```",
        language,
        &code[0..std::cmp::min(code.len(), 1980)]
    )
}

#[cfg(target_os = "linux")]
pub fn get_memory_usage() -> usize {
    // todo
    0
}

#[cfg(not(target_os = "linux"))]
pub fn get_memory_usage() -> usize {
    0
}

pub async fn download_content(
    client: &reqwest::Client,
    url: &str,
    limit_bytes: usize,
) -> Result<Vec<u8>, String> {
    let mut stream = client
        .get(url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .bytes_stream();

    let mut data = Vec::with_capacity(limit_bytes);
    while let Some(chunk) = stream.next().await.and_then(|x| x.ok()) {
        for byte in chunk {
            if data.len() > limit_bytes {
                return Err(format!(
                    "The download exceeded the specified limit of {} bytes",
                    limit_bytes
                ));
            }
            data.push(byte);
        }
    }

    Ok(data)
}

mod units {
    pub const SECOND: u64 = 1000;
    pub const MINUTE: u64 = SECOND * 60;
    pub const HOUR: u64 = MINUTE * 60;
    pub const DAY: u64 = HOUR * 24;
}

pub fn pluralize<'a>(s: &'a str, adder: &str, count: u64) -> Cow<'a, str> {
    if count == 1 {
        Cow::Borrowed(s)
    } else {
        Cow::Owned(s.to_owned() + adder)
    }
}

pub struct Uptime(u64);
impl Uptime {
    pub fn new(time: u64) -> Self {
        Self(time)
    }

    pub fn format(&self) -> String {
        let time = self.0;
        
        if time >= units::DAY {
            let amount = time / units::DAY;
            format!("{} {}", amount, pluralize("day", "s", amount))
        } else if time >= units::HOUR {
            let amount = time / units::HOUR;
            format!("{} {}", amount, pluralize("hour", "s", amount))
        } else if time >= units::MINUTE {
            let amount = time / units::MINUTE;
            format!("{} {}", amount, pluralize("minute", "s", amount))
        } else {
            let amount = time / units::SECOND;
            format!("{} {}", amount, pluralize("second", "s", amount))
        }
    }
}

fn unit_to_ms(u: &str) -> u32 {
    match u {
        "s" => 1000,
        "m" => 1000 * 60,
        "h" => 1000 * 60 * 60,
        "d" => 1000 * 60 * 60 * 24,
        _ => unreachable!()
    }
}

pub fn parse_to_millis(input: &str) -> Result<u32, ParseIntError> {
    let matches = regexes::TIME_STRING.captures_iter(input);

    let mut total = 0u32;

    for current in matches {
        let amount = current[1].parse::<u32>()?;
        let unit = unit_to_ms(&current[2]);

        total += amount * unit;
    }
    
    Ok(total)
}

// Ugly solution for now
// Twilight currently doesn't support Allowed Mentions API for Webhooks
// TODO: Use allowed_mentions once it's out
pub fn sanitize_message_content(content: &str) -> String {
    content.replace("@", "@\u{200b}")
}