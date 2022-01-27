use crate::{assyst::Assyst, command::context::Context, filetype, rest::wsi::RequestError};
use assyst_common::consts;
use bytes::Bytes;
use futures_util::StreamExt;
use regex::Captures;
use shared::job::JobResult;

use std::{
    borrow::Cow,
    convert::TryInto,
    num::ParseIntError,
    process::Command,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use twilight_http::{error::Error, Client};
use twilight_model::{
    channel::{message::Mention, Channel},
    guild::Permissions,
    id::{GuildId, UserId},
};

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
        pub static ref CUSTOM_EMOJI: Regex = Regex::new(r"<a?:(\w+):(\d{16,20})>").unwrap();
        pub static ref TENOR_GIF: Regex = Regex::new(r"https://\w+\.tenor\.com/[\w\-]+/[^\.]+\.gif").unwrap();
        pub static ref URL: Regex = Regex::new(r"https?://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)").unwrap();
        pub static ref USER_MENTION: Regex = Regex::new(r"(?:<@!?)?(\d{16,20})>?").unwrap();
        pub static ref TIME_STRING: Regex = Regex::new("(\\d+)([smhd])").unwrap();
        pub static ref COMMAND_FLAG: Regex = Regex::new(r#"\s+-(\w+)(?: *"([^"]+)"| *([^\-\s]+))?"#).unwrap();
    }
}

/// Returns the file type given [`Bytes`]
pub fn get_buffer_filetype(buffer: &Bytes) -> Option<&'static str> {
    Some(filetype::get_sig(&buffer.to_vec())?.as_str())
}

/// Returns the current timestamp in milliseconds
pub fn get_current_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
        .try_into()
        .expect("Couldn't fit timestamp (u128) into i64")
}

/// Returns the longer string of the two given strings
pub fn get_longer_str<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

/// Generates a table given a list of tuples containing strings
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

pub async fn ensure_same_guild(
    context: &Arc<Context>,
    channel_id: u64,
    guild_id: u64,
) -> Result<(), String> {
    let is = is_same_guild(context.http(), channel_id, guild_id)
        .await
        .map_err(|e| e.to_string())?;

    if !is {
        return Err(String::from(
            "The provided channel is not part of this guild.",
        ));
    } else {
        Ok(())
    }
}

pub async fn is_same_guild(
    client: &Client,
    channel_id: u64,
    guild_id: u64,
) -> Result<bool, Box<dyn std::error::Error>> {
    let real_guild_id = client
        .channel(channel_id.into())
        .await?
        .and_then(|c| match c {
            Channel::Guild(g) => g.guild_id(),
            _ => None,
        })
        .map(|g| g.0);

    Ok(real_guild_id == Some(guild_id))
}

/// Generates a list given a list of tuples containing strings
pub fn generate_list<K: AsRef<str>, V: AsRef<str>>(
    key_name: &str,
    value_name: &str,
    values: &[(K, V)],
) -> String {
    let longest = get_longer_str(
        key_name,
        values
            .iter()
            .fold(values[0].0.as_ref(), |previous, (current, _)| {
                get_longer_str(previous, current.as_ref())
            }),
    );

    let mut output = format!(
        " {4}{}\t{}\n {4}{}\t{}",
        key_name,
        value_name,
        "-".repeat(key_name.len()),
        "-".repeat(value_name.len()),
        " ".repeat(longest.len() - key_name.len()),
    );

    let formatted_values = values
        .iter()
        .map(|(k, v)| {
            format!(
                " {}{}\t{}",
                " ".repeat(longest.len() - k.as_ref().len()),
                k.as_ref(),
                v.as_ref()
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    output = format!("{}\n{}", output, formatted_values);

    output
}

/// Wraps `code` in a Discord codeblock
pub fn codeblock(code: &str, language: &str) -> String {
    let escaped_code = code.replace("`", "`\u{0200b}");
    format!(
        "```{}\n{}\n```",
        language,
        escaped_code.chars().take(1980).collect::<String>()
    )
}

pub const CODEBLOCK_MD: &str = "```";

/// Parses a codeblock
pub fn parse_codeblock<'a>(text: &'a str, lang: &str) -> &'a str {
    if !text.starts_with(&format!("{}{}", CODEBLOCK_MD, lang))
        || !text.ends_with(CODEBLOCK_MD)
        || text.len() <= CODEBLOCK_MD.len() * 2
    {
        text
    } else {
        &text[(lang.len() + CODEBLOCK_MD.len())..(text.len() - CODEBLOCK_MD.len())].trim()
    }
}

/// Parses a codeblock and its language
pub fn parse_codeblock_with_language(text: &str) -> Option<(&str, &str)> {
    let text = text.trim();

    let stripped = text.strip_prefix("```")?;

    let lang_end = stripped.chars().position(|x| !x.is_ascii_alphabetic())?;

    let lang = &stripped[..lang_end];
    let code = stripped.get(lang_end + 1..)?.strip_suffix("```")?;

    Some((lang, code))
}

/// Attempts to extract memory usage
#[cfg(target_os = "linux")]
pub fn get_memory_usage() -> Option<usize> {
    use std::fs;

    let field = 1;
    let contents = fs::read("/proc/self/statm").ok()?;
    let contents = String::from_utf8(contents).ok()?;
    let s = contents.split_whitespace().nth(field)?;
    let npages = s.parse::<usize>().ok()?;
    Some(npages * 4096)

    /*
    use std::{ffi::{CStr, c_void}, os::raw::c_char};

    use serde::Deserialize;

    unsafe {
        use jemalloc_sys::malloc_stats_print;
        let mut output = String::new();

        unsafe extern "C" fn write_cb(output: *mut c_void, buf: *const c_char) {
            let buf = CStr::from_ptr(buf).to_str().unwrap();
            let output = &mut *output.cast::<String>();
            output.push_str(buf);
        }
        
        let opts = b"Jgmdablx\0".as_ptr().cast::<i8>();

        malloc_stats_print(Some(write_cb), &mut output as *mut String as *mut c_void, opts);

        #[derive(Deserialize)]
        struct Stats {
            pub resident: usize
        }
        #[derive(Deserialize)]
        struct Jemalloc {
            pub stats: Stats
        }
        #[derive(Deserialize)]
        struct JemallocStats {
            pub jemalloc: Jemalloc
        }

        let json: JemallocStats = serde_json::from_str(&output).unwrap();

        Some(json.jemalloc.stats.resident)
    }*/
}

#[cfg(not(target_os = "linux"))]
pub fn get_memory_usage() -> Option<usize> {
    use std::mem::{self, MaybeUninit};
    use winapi::shared::minwindef::DWORD;
    use winapi::um::processthreadsapi::GetCurrentProcess;
    use winapi::um::psapi::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS};

    let mut pmc = MaybeUninit::<PROCESS_MEMORY_COUNTERS>::uninit();
    match unsafe {
        GetProcessMemoryInfo(GetCurrentProcess(), pmc.as_mut_ptr(), mem::size_of_val(&pmc) as DWORD)
    } {
        0 => None,
        _ => {
            let pmc = unsafe { pmc.assume_init() };
            Some(pmc.WorkingSetSize as usize)
        }
    }
}

// Get memory usage in MB
pub fn get_memory_usage_num() -> Option<f32> {
    let memory = get_memory_usage()? as f32/1000f32/1000f32;

    Some(memory)
}

/// Attempts to download the content of a url
pub async fn download_content(
    client: &reqwest::Client,
    url: &str,
    limit_bytes: usize,
) -> Result<Vec<u8>, String> {
    let request = client
        .get(url)
        .header(reqwest::header::USER_AGENT, "Assyst Content Downloader")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let status = request.status();
    if status != reqwest::StatusCode::OK {
        return Err(format!("Download failed: {}", status));
    }

    let mut stream = request.bytes_stream();

    let mut data = Vec::with_capacity(limit_bytes);
    while let Some(chunk) = stream.next().await.and_then(|x| x.ok()) {
        for byte in chunk {
            if data.len() > limit_bytes {
                return Err(format!(
                    "The download exceeded the specified limit of {}MB",
                    limit_bytes / 1000usize.pow(2)
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

/// Pluralizes a string
pub fn pluralize<'a>(s: &'a str, adder: &str, count: u64) -> Cow<'a, str> {
    if count == 1 {
        Cow::Borrowed(s)
    } else {
        Cow::Owned(s.to_owned() + adder)
    }
}

/// A wrapper around uptime
pub struct Uptime(pub u64);
impl Uptime {
    pub fn new(time: u64) -> Self {
        Self(time)
    }

    /// Formats uptime
    pub fn format(&self) -> String {
        format_time(self.0)
    }
}

/// Converts a unit string (s, m, h, d) to milliseconds
fn unit_to_ms(u: &str) -> u64 {
    match u {
        "s" => 1000,
        "m" => 1000 * 60,
        "h" => 1000 * 60 * 60,
        "d" => 1000 * 60 * 60 * 24,
        _ => unreachable!(),
    }
}

/// Parses a string to milliseconds
pub fn parse_to_millis(input: &str) -> Result<u64, ParseIntError> {
    let matches = regexes::TIME_STRING.captures_iter(input);

    let mut total = 0;

    for current in matches {
        let amount = current[1].parse::<u64>()?;
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

/// Attempts to return the timestamp as a Discord timestamp,
/// and falls back to [`format_time`] if Discord were to render it as "Invalid Date"
pub fn format_discord_timestamp(input: u64) -> String {
    if input <= consts::MAX_TIMESTAMP {
        format!("<t:{}:R>", input / 1000)
    } else {
        format_time(input)
    }
}

/// Converts a timestamp to a humanly readable string
pub fn format_time(input: u64) -> String {
    if input >= units::DAY {
        let amount = input / units::DAY;
        format!("{} {}", amount, pluralize("day", "s", amount))
    } else if input >= units::HOUR {
        let amount = input / units::HOUR;
        format!("{} {}", amount, pluralize("hour", "s", amount))
    } else if input >= units::MINUTE {
        let amount = input / units::MINUTE;
        format!("{} {}", amount, pluralize("minute", "s", amount))
    } else {
        let amount = input / units::SECOND;
        format!("{} {}", amount, pluralize("second", "s", amount))
    }
}

/// Normalizes custom emojis by replacing them with their names
pub fn normalize_emojis(input: &str) -> Cow<'_, str> {
    regexes::CUSTOM_EMOJI.replace_all(input, |c: &Captures| c.get(1).unwrap().as_str().to_string())
}

/// Normalizes mentions by replacing them with their names
pub fn normalize_mentions<'a>(input: &'a str, mentions: &[Mention]) -> Cow<'a, str> {
    regexes::USER_MENTION.replace_all(input, |c: &Captures| {
        let id = c.get(1).unwrap().as_str();
        let name = mentions
            .iter()
            .find(|m| m.id.to_string().eq(id))
            .map(|m| m.name.clone())
            .unwrap_or_else(String::new);
        name
    })
}

/// Attempts to extract the page title
pub fn extract_page_title(input: &str) -> Option<String> {
    let dom = tl::parse(input, tl::ParserOptions::default());
    let parser = dom.parser();

    let tag = dom.query_selector("title")?.next()?.get(parser)?;

    Some(tag.inner_text(parser).into_owned())
}

/// Generates a message link
pub fn message_link(guild_id: u64, channel_id: u64, message_id: u64) -> String {
    format!(
        "https://discord.com/channels/{}/{}/{}",
        guild_id, channel_id, message_id
    )
}

pub struct CommandOutput {
    pub stdout: String,
    pub stderr: String,
}

/// Executes a bash command
pub fn exec_sync(command: &str) -> Result<CommandOutput, std::io::Error> {
    let mut cmd = Command::new("bash");
    cmd.args(&["-c", command]);

    let output = cmd.output()?;

    Ok(CommandOutput {
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
    })
}

/// Attempts to resolve the guild owner
pub async fn get_guild_owner(http: &Client, guild_id: GuildId) -> Result<UserId, Error> {
    Ok(http.guild(guild_id).await?.unwrap().owner_id)
}

pub async fn is_guild_manager(
    http: &Client,
    guild_id: GuildId,
    user_id: UserId,
) -> Result<bool, Error> {
    // guild owner *or* manage server *or* admin
    // get owner
    let owner = get_guild_owner(http, guild_id).await?;

    // figure out permissions of the user through bitwise operations
    let member = http.guild_member(guild_id, user_id).await?.unwrap();

    let roles = http.roles(guild_id).await?;

    let member_roles = roles
        .iter()
        .filter(|r| member.roles.contains(&r.id))
        .collect::<Vec<_>>();

    let member_permissions = member_roles.iter().fold(0, |a, r| a | r.permissions.bits());
    let member_is_manager = member_permissions & Permissions::ADMINISTRATOR.bits()
        == Permissions::ADMINISTRATOR.bits()
        || member_permissions & Permissions::MANAGE_GUILD.bits()
            == Permissions::MANAGE_GUILD.bits();

    Ok(owner == user_id || member_is_manager)
}

pub async fn ensure_guild_manager(
    context: &Arc<Context>,
    guild_id: impl Into<GuildId>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let user_id = context.message.author.id;

    if is_guild_manager(context.http(), guild_id.into(), user_id).await? {
        Ok(())
    } else {
        Err("You need manage server permissions to run this command".into())
    }
}

/// Converts number of bytes to a humanly readable string
pub fn bytes_to_readable(bytes: usize) -> String {
    if bytes > 1000usize.pow(2) {
        format!("{:.2}MB", (bytes as f32 / 1000f32.powi(2)))
    } else if bytes > 1000 {
        format!("{:.2}KB", (bytes as f32 / 1000f32))
    } else {
        format!("{}B", bytes)
    }
}

/// This function will remove a free voter request if the user has any
/// and are not a patron!
pub async fn get_wsi_request_tier(assyst: &Assyst, user_id: UserId) -> Result<usize, sqlx::Error> {
    let patrons = assyst.patrons.read().await;
    let patron = patrons.iter().find(|i| i.user_id == user_id);
    if let Some(p) = patron {
        return Ok(p.tier);
    }

    let has_free_tier_1 = assyst
        .database
        .get_and_subtract_free_tier_1_request(user_id.0 as i64)
        .await?;

    if has_free_tier_1 {
        Ok(1)
    } else {
        Ok(0)
    }
}

/// Formats a number of nanoseconds to a humanly readable string
pub fn nanos_to_readable(time: u32) -> String {
    if time < 1_000 {
        return format!("{}ns", time);
    } else if time < 1_000_000 {
        return format!("{:.2}µs", (time as f64) / 1_000f64);
    } else if time < 1_000_000_000 {
        return format!("{:.2}ms", (time as f64) / 1_000_000f64);
    } else {
        return format!("{:.2}s", (time as f64) / 1_000_000_000f64);
    }
}

pub fn handle_job_result(result: JobResult) -> Result<Bytes, RequestError> {
    match result.result {
        Ok(data) => Ok(Bytes::from(data)),
        Err(err) => Err(RequestError::from(err)),
    }
}