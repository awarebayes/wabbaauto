use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct LoginStatus {
    pub chrome: bool,
    pub nexusmods: bool,
    pub loverslab: bool,
    pub error: String,
}

#[derive(Debug, Serialize)]

pub struct DownloadReponse {
    pub error: String,
    pub success: bool,
}
