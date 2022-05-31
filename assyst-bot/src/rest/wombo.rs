use reqwest::Error;
use serde::Deserialize;
use serde_repr::Deserialize_repr;
use std::str::FromStr;

use crate::assyst::Assyst;

#[derive(Deserialize)]
pub struct WomboResponseResult {
    #[serde(rename = "final")]
    pub url: String,
}

#[derive(Deserialize)]
pub struct WomboResponse {
    pub result: WomboResponseResult,
}

// https://github.com/y21/wombo-dream-api/blob/server/src/server.ts#L9
#[derive(Deserialize_repr, Debug)]
#[repr(u8)]
pub enum WomboErrorCode {
    MalformedQuery = 0,
    Timeout = 1,
    Ratelimit = 2,
    TaskFail = 3,
}

impl std::fmt::Display for WomboErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WomboErrorCode::MalformedQuery => write!(f, "Malformed query parameters"),
            WomboErrorCode::Timeout => write!(f, "Task timed out"),
            WomboErrorCode::Ratelimit => write!(f, "Bot is currently rate limited"),
            WomboErrorCode::TaskFail => write!(f, "Task failed due to an unknown reason"),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct WomboErrorResponse {
    pub code: WomboErrorCode,
}

#[derive(Clone, Copy)]
pub enum WomboStyle {
    Psychedelic = 21,
    Surreal = 23,
    Synthwave = 1,
    Ghibli = 22,
    Steampunk = 4,
    Fantasy = 5,
    Vibrant = 6,
    Hd = 7,
    Psychic = 9,
    DarkFantasy = 10,
    Mystical = 11,
    Baroque = 13,
    Etching = 14,
    Sdali = 15,
    Wuhtercuhler = 16,
    Provenance = 17,
    Moonwalker = 19,
    Blacklight = 20,
    None = 3,
    Ukiyoe = 2,
    RoseGold = 18,
}

static WOMBO_STYLES: phf::Map<&'static str, WomboStyle> = phf_macros::proc_macro_hack_phf_map! {
    "psychedelic" => WomboStyle::Psychedelic,
    "surreal" => WomboStyle::Surreal,
    "synthwave" => WomboStyle::Synthwave,
    "ghibli" => WomboStyle::Ghibli,
    "steampunk" => WomboStyle::Steampunk,
    "fantasy" => WomboStyle::Fantasy,
    "vibrant" => WomboStyle::Vibrant,
    "hd" => WomboStyle::Hd,
    "psychic" => WomboStyle::Psychic,
    "darkfantasy" => WomboStyle::DarkFantasy,
    "mystical" => WomboStyle::Mystical,
    "baroque" => WomboStyle::Baroque,
    "etching" => WomboStyle::Etching,
    "sdali" => WomboStyle::Sdali,
    "wuhtercuhler" => WomboStyle::Wuhtercuhler,
    "provenance" => WomboStyle::Provenance,
    "moonwalker" => WomboStyle::Moonwalker,
    "blacklight" => WomboStyle::Blacklight,
    "none" => WomboStyle::None,
    "ukiyoe" => WomboStyle::Ukiyoe,
    "rosegold" => WomboStyle::RoseGold,
};

#[derive(Debug)]
pub struct UnknownStyle;

impl std::fmt::Display for UnknownStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unknown style provided. Valid styles: ")?;

        for (index, key) in WOMBO_STYLES.keys().enumerate() {
            if index > 0 {
                write!(f, ", ")?;
            }

            write!(f, "{key}")?;
        }

        Ok(())
    }
}

impl std::error::Error for UnknownStyle {}

impl FromStr for WomboStyle {
    type Err = UnknownStyle;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        WOMBO_STYLES.get(s).copied().ok_or(UnknownStyle)
    }
}

#[derive(Debug)]
pub enum WomboError {
    Reqwest(Error),
    Wombo(WomboErrorCode),
}

impl From<Error> for WomboError {
    fn from(e: Error) -> Self {
        WomboError::Reqwest(e)
    }
}
impl std::fmt::Display for WomboError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WomboError::Reqwest(e) => write!(f, "Reqwest error: {e}"),
            WomboError::Wombo(s) => write!(f, "Wombo error: {s}"),
        }
    }
}
impl std::error::Error for WomboError {}

pub async fn generate(
    assyst: &Assyst,
    style: WomboStyle,
    prompt: &str,
) -> Result<WomboResponse, WomboError> {
    let style = style as u8;
    let url = assyst.config.url.wombo.as_ref();

    let req = assyst
        .reqwest_client
        .get(url)
        .query(&[
            ("style", style.to_string()),
            ("message", prompt.to_string()),
        ])
        .send()
        .await?;

    if req.status().is_success() {
        let resp = req.json::<WomboResponse>().await?;
        Ok(resp)
    } else {
        let resp = req.json::<WomboErrorResponse>().await?;
        Err(WomboError::Wombo(resp.code))
    }
}
