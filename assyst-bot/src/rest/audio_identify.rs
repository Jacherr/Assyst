use std::sync::Arc;

use crate::assyst::Assyst;

pub mod songdetection {
    use serde::{Deserialize, Serialize};
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Root {
        pub matches: Vec<Match>,
        pub timestamp: i64,
        pub timezone: String,
        pub tagid: String,
        pub track: Option<Track>,
    }
    
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Match {
        pub id: String,
        pub offset: f64,
        pub channel: Option<String>,
        pub timeskew: f64,
        pub frequencyskew: f64,
    }
    
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Track {
        pub layout: String,
        #[serde(rename = "type")]
        pub type_field: String,
        pub key: String,
        pub title: String,
        pub subtitle: String,
        pub images: Images,
        pub share: Share,
        pub hub: Hub,
        pub url: String,
        pub artists: Vec<Artist>,
        pub isrc: Option<String>,
        pub genres: Genres,
        pub urlparams: Urlparams,
        pub myshazam: Option<Myshazam>,
        pub albumadamid: Option<String>,
        pub sections: Vec<Section>,
    }
    
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Images {
        pub background: String,
        pub coverart: String,
        pub coverarthq: String,
        pub joecolor: Option<String>,
    }
    
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Share {
        pub subject: String,
        pub text: String,
        pub href: String,
        pub image: String,
        pub twitter: String,
        pub html: String,
        pub avatar: Option<String>,
        pub snapchat: String,
    }
    
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Hub {
        #[serde(rename = "type")]
        pub type_field: String,
        pub image: String,
        pub actions: Option<Vec<Action>>,
        pub options: Vec<Option1>,
        pub providers: Vec<Provider>,
        pub explicit: bool,
        pub displayname: String,
    }
    
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Action {
        pub name: String,
        #[serde(rename = "type")]
        pub type_field: String,
        pub id: Option<String>,
        pub uri: Option<String>,
    }
    
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Option1 {
        pub caption: String,
        pub actions: Option<Vec<Action2>>,
        pub beacondata: Beacondata,
        pub image: String,
        #[serde(rename = "type")]
        pub type_field: String,
        pub listcaption: String,
        pub overflowimage: String,
        pub colouroverflowimage: bool,
        pub providername: String,
    }
    
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Action2 {
        #[serde(rename = "type")]
        pub type_field: String,
        pub uri: String,
        pub name: Option<String>,
    }
    
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Beacondata {
        #[serde(rename = "type")]
        pub type_field: String,
        pub providername: String,
    }
    
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Provider {
        pub caption: String,
        pub images: Images2,
        pub actions: Option<Vec<Action3>>,
        #[serde(rename = "type")]
        pub type_field: String,
    }
    
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Images2 {
        pub overflow: String,
        pub default: String,
    }
    
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Action3 {
        pub name: String,
        #[serde(rename = "type")]
        pub type_field: String,
        pub uri: String,
    }
    
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Artist {
        pub id: String,
        pub adamid: String,
    }
    
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Genres {
        pub primary: String,
    }
    
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Urlparams {
        #[serde(rename = "{tracktitle}")]
        pub tracktitle: String,
        #[serde(rename = "{trackartist}")]
        pub trackartist: String,
    }
    
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Myshazam {
        pub apple: Apple,
    }
    
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Apple {
        pub actions: Option<Vec<Action4>>,
    }
    
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Action4 {
        pub name: String,
        #[serde(rename = "type")]
        pub type_field: String,
        pub uri: String,
    }
    
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Section {
        #[serde(rename = "type")]
        pub type_field: String,
        pub metapages: Option<Vec<Metapage>>,
        pub tabname: String,
        pub metadata: Option<Vec<Metadaum>>,
        #[serde(default)]
        pub text: Vec<String>,
        pub footer: Option<String>,
        pub beacondata: Option<Beacondata2>,
        pub youtubeurl: Option<Youtubeurl>,
        pub avatar: Option<String>,
        pub id: Option<String>,
        pub name: Option<String>,
        pub verified: Option<bool>,
        pub actions: Option<Vec<Action6>>,
    }
    
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Metapage {
        pub image: String,
        pub caption: String,
    }
    
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Metadaum {
        pub title: String,
        pub text: String,
    }
    
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Beacondata2 {
        pub lyricsid: String,
        pub providername: String,
        pub commontrackid: String,
    }
    
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Youtubeurl {
        pub caption: String,
        pub image: Image,
        pub actions: Option<Vec<Action5>>,
    }
    
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Image {
        pub dimensions: Dimensions,
        pub url: String,
    }
    
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Dimensions {
        pub width: i64,
        pub height: i64,
    }
    
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Action5 {
        pub name: String,
        #[serde(rename = "type")]
        pub type_field: String,
        pub share: Share2,
        pub uri: String,
    }
    
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Share2 {
        pub subject: String,
        pub text: String,
        pub href: String,
        pub image: String,
        pub twitter: String,
        pub html: String,
        pub avatar: Option<String>,
        pub snapchat: String,
    }
    
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Action6 {
        #[serde(rename = "type")]
        pub type_field: String,
        pub id: String,
    }
}


pub mod songsearch {
    use serde::{Deserialize, Serialize};
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Root {
        pub tracks: Option<Tracks>,
        pub artists: Option<Artists>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Tracks {
        pub hits: Vec<Hit>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Hit {
        pub track: Track,
        pub snippet: Option<String>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Track {
        pub layout: String,
        #[serde(rename = "type")]
        pub type_field: String,
        pub key: String,
        pub title: String,
        pub subtitle: String,
        pub share: Share,
        pub images: Images,
        pub hub: Hub,
        pub artists: Vec<Artist>,
        pub url: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Share {
        pub subject: String,
        pub text: String,
        pub href: String,
        pub image: String,
        pub twitter: String,
        pub html: String,
        pub avatar: Option<String>,
        pub snapchat: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Images {
        pub background: String,
        pub coverart: String,
        pub coverarthq: String,
        pub joecolor: Option<String>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Hub {
        #[serde(rename = "type")]
        pub type_field: String,
        pub image: String,
        pub actions: Vec<Action>,
        pub options: Vec<Option1>,
        pub providers: Vec<Provider>,
        pub explicit: bool,
        pub displayname: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Action {
        pub name: String,
        #[serde(rename = "type")]
        pub type_field: String,
        pub id: Option<String>,
        pub uri: Option<String>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Option1 {
        pub caption: String,
        pub actions: Vec<Action2>,
        pub beacondata: Beacondata,
        pub image: String,
        #[serde(rename = "type")]
        pub type_field: String,
        pub listcaption: String,
        pub overflowimage: String,
        pub colouroverflowimage: bool,
        pub providername: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Action2 {
        #[serde(rename = "type")]
        pub type_field: String,
        pub uri: String,
        pub name: Option<String>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Beacondata {
        #[serde(rename = "type")]
        pub type_field: String,
        pub providername: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Provider {
        pub caption: String,
        pub images: Images2,
        pub actions: Vec<Action3>,
        #[serde(rename = "type")]
        pub type_field: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Images2 {
        pub overflow: String,
        pub default: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Action3 {
        pub name: String,
        #[serde(rename = "type")]
        pub type_field: String,
        pub uri: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Artist {
        pub id: String,
        pub adamid: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Artists {
        pub hits: Vec<Hit2>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Hit2 {
        pub artist: Artist2,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Artist2 {
        pub avatar: Option<String>,
        pub name: String,
        pub verified: bool,
        pub weburl: String,
        pub adamid: String,
    }
}

const DETECT_URL: &str = "https://shazam.p.rapidapi.com/songs/detect";
const SEARCH_URL: &str = "https://shazam.p.rapidapi.com/search";

pub async fn identify_audio(assyst: Arc<Assyst>, base64: String) -> anyhow::Result<songdetection::Root> {
    let client = &assyst.reqwest_client;
    Ok(client
        .post(DETECT_URL)
        .header(
            "X-RapidAPI-Key",
            assyst.config.auth.rapidapi_shazam.to_string(),
        )
        .header("X-RapidAPI-host", "shazam.p.rapidapi.com")
        .header("content-type", "text/plain")
        .body(base64)
        .send()
        .await?
        .error_for_status()?
        .json::<songdetection::Root>()
        .await?)
}

pub async fn search_song(assyst: Arc<Assyst>, search: String) -> anyhow::Result<songsearch::Root> {
    let client = &assyst.reqwest_client;
    Ok(client
        .get(SEARCH_URL)
        .header(
            "X-RapidAPI-Key",
            assyst.config.auth.rapidapi_shazam.to_string(),
        )
        .header("X-RapidAPI-host", "shazam.p.rapidapi.com")
        .query(&[("term", &search[..])])
        .send()
        .await?
        .error_for_status()?
        .json::<songsearch::Root>()
        .await?)
}
