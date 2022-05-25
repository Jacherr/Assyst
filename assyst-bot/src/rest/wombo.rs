use std::str::FromStr;

use serde::Deserialize;

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

#[derive(Clone, Copy)]
pub enum WomboStyle {
    Psychadelic = 21,
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
    Wuhercuhler = 16,
    Provenance = 17,
    MoonWalker = 19,
    BlackLight = 20,
    None = 3,
    Ukiyoe = 2,
    RoseGold = 18,
}

impl FromStr for WomboStyle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "psychadelic" => Ok(WomboStyle::Psychadelic),
            "surreal" => Ok(WomboStyle::Surreal),
            "synthwave" => Ok(WomboStyle::Synthwave),
            "ghibli" => Ok(WomboStyle::Ghibli),
            "steampunk" => Ok(WomboStyle::Steampunk),
            "fantasy" => Ok(WomboStyle::Fantasy),
            "vibrant" => Ok(WomboStyle::Vibrant),
            "hd" => Ok(WomboStyle::Hd),
            "psychic" => Ok(WomboStyle::Psychic),
            "darkfantasy" => Ok(WomboStyle::DarkFantasy),
            "mystical" => Ok(WomboStyle::Mystical),
            "baroque" => Ok(WomboStyle::Baroque),
            "etching" => Ok(WomboStyle::Etching),
            "sdali" => Ok(WomboStyle::Sdali),
            "wuhercuhler" => Ok(WomboStyle::Wuhercuhler),
            "provenance" => Ok(WomboStyle::Provenance),
            "moonwalker" => Ok(WomboStyle::MoonWalker),
            "blacklight" => Ok(WomboStyle::BlackLight),
            "none" => Ok(WomboStyle::None),
            "ukiyoe" => Ok(WomboStyle::Ukiyoe),
            "rosegold" => Ok(WomboStyle::RoseGold),
            _ => Err(()),
        }
    }
}

pub const STYLE_LIST: &[&str] = &[
    "psychadelic",
    "surreal",
    "synthwave",
    "ghibli",
    "steampunk",
    "fantasy",
    "vibrant",
    "hd",
    "psychic",
    "darkfantasy",
    "mystical",
    "baroque",
    "etching",
    "sdali",
    "wuhercuhler",
    "provenance",
    "moonwalker",
    "blacklight",
    "none",
    "ukiyoe",
    "rosegold",
];

pub async fn generate(
    assyst: &Assyst,
    style: WomboStyle,
    prompt: &str,
) -> reqwest::Result<WomboResponse> {
    let style = style as u8;
    let url = assyst.config.url.wombo.as_ref();

    assyst
        .reqwest_client
        .get(url)
        .query(&[
            ("style", style.to_string()),
            ("message", prompt.to_string()),
        ])
        .send()
        .await?
        .json()
        .await
}
