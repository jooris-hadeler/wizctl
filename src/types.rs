use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Method {
    GetState,
    SetState,
    GetPilot,
    SetPilot,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub method: Method,
    pub params: T,
}

impl<T> Request<T> {
    pub fn new(id: i64, method: Method, params: T) -> Self {
        Self {
            id: Some(id),
            method,
            params,
        }
    }

    pub fn new_without_id(method: Method, params: T) -> Self {
        Self {
            id: None,
            method,
            params,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorParams {
    #[serde(rename = "r")]
    pub red: u8,
    #[serde(rename = "g")]
    pub green: u8,
    #[serde(rename = "b")]
    pub blue: u8,
    pub dimming: u8,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneParams {
    #[serde(rename = "sceneId")]
    pub scene: Scene,
    pub speed: u8,
    pub dimming: u8,
}

#[repr(i64)]
#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, Serialize_repr, Deserialize_repr, clap::ValueEnum)]
#[clap(rename_all="kebab_case")]
pub enum Scene {
    Ocean        = 1,
    Romance      = 2,
    Sunset       = 3,
    Party        = 4,
    Fireplace    = 5,
    Cozy         = 6,
    Forest       = 7,
    PastelColors = 8,
    WakeUp       = 9,
    Bedtime      = 10,
    WarmWhite    = 11,
    Daylight     = 12,
    CoolWhite    = 13,
    NightLight   = 14,
    Focus        = 15,
    Relax        = 16,
    TrueColors   = 17,
    TvTime       = 18,
    Plantgrowth  = 19,
    Spring       = 20,
    Summer       = 21,
    Fall         = 22,
    Deepdive     = 23,
    Jungle       = 24,
    Mojito       = 25,
    Club         = 26,
    Christmas    = 27,
    Halloween    = 28,
    Candlelight  = 29,
    GoldenWhite  = 30,
    Pulse        = 31,
    Steampunk    = 32,
    Rhythm       = 1000,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StateParams {
    pub state: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmptyParams;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response<T> {
    pub id: Option<i64>,
    pub method: String,
    pub env: String,
    pub result: T,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub mac: String,
    pub rssi: i64,
    pub state: bool,
    pub scene_id: i64,
    #[serde(default)]
    pub r: i64,
    #[serde(default)]
    pub g: i64,
    #[serde(default)]
    pub b: i64,
    #[serde(default)]
    pub c: i64,
    #[serde(default)]
    pub w: i64,
    #[serde(default)]
    pub speed: i64,
    pub dimming: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandResult {
    pub success: bool,
}
