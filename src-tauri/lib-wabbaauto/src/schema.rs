use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct ModOriginalState {
    #[serde(rename = "$type")]
    pub type_field: String,

    #[serde(rename = "ModID")]
    pub mod_id: Option<usize>,

    #[serde(rename = "FileID")]
    pub file_id: Option<usize>,

    #[serde(rename = "GameName")]
    pub game_name: Option<String>,

    #[serde(rename = "Name")]
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ModOriginal {
    #[serde(rename = "State")]
    pub state: ModOriginalState,

    #[serde(rename = "Hash")]
    pub hash: String,

    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Mod {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Original")]
    pub original: ModOriginal,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ModlistStatus {
    #[serde(rename = "Archives")]
    pub archives: Vec<Mod>,
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Version")]
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginStatus {
    pub chrome: bool,
    pub website: bool,
    pub error: String,
}
