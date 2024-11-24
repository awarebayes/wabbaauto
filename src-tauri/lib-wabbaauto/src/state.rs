use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", content = "state")]
pub enum DownloadState {
    Init,
    Started,
    InProgress {
        percent: usize,
        speed_kbps: usize,
        downloaded: usize,
        total_size: usize,
    },
    Ended,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "kind", content = "state")]
pub enum GettingLinkState {
    Init,
    ConnectedToBrowser,
    WaitingForLink,
    Ended,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "kind", content = "state")]
pub enum HashState {
    Init,
    Started,
    InProgress { percent: usize },
    Ended,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "kind", content = "state")]
pub enum FailedState {
    Downloading,
    Hashing,
    Unknown(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "kind", content = "state")]
pub enum ModGetState {
    Init,
    GettingLink(GettingLinkState),
    Downloading(DownloadState),
    Hashing(HashState),
    Ended,
    Failed(FailedState),
}

impl ModGetState {
    pub fn has_ended(&self) -> bool {
        match self {
            ModGetState::Ended => true,
            _ => false,
        }
    }

    pub fn has_failed(&self) -> bool {
        match self {
            ModGetState::Failed(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AppState {
    pub getting_archives: Vec<(String, ModGetState)>,
    pub recent_fails: Vec<String>,
    pub total: usize,
    pub failed: usize,
    pub successes: usize,
}

impl fmt::Display for ModGetState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ModGetState::Init => write!(f, "Initialized"),
            ModGetState::GettingLink(GettingLinkState::Init) => write!(f, "Gettinig link / init"),
            ModGetState::GettingLink(GettingLinkState::ConnectedToBrowser) => {
                write!(f, "Gettinig link / Connected to browser")
            }
            ModGetState::GettingLink(GettingLinkState::WaitingForLink) => {
                write!(f, "Gettinig link / Waiting for link")
            }
            ModGetState::GettingLink(GettingLinkState::Ended) => {
                write!(f, "Gettinig tting link / ended")
            }
            ModGetState::Downloading(DownloadState::Init) => write!(f, "Downloading / init"),
            ModGetState::Downloading(DownloadState::Started) => write!(f, "Downloading / started"),
            ModGetState::Downloading(DownloadState::InProgress {
                percent,
                speed_kbps,
                downloaded,
                total_size,
            }) => write!(f, "Downloading / {} % {} kb/s", percent, speed_kbps),
            ModGetState::Downloading(DownloadState::Ended) => write!(f, "Downloading / ended"),
            ModGetState::Hashing(HashState::Init) => write!(f, "Hasning / init"),
            ModGetState::Hashing(HashState::Started) => write!(f, "Hasning / started"),
            ModGetState::Hashing(HashState::InProgress { percent }) => {
                write!(f, "Hasning / in progress {}%", percent)
            }
            ModGetState::Hashing(HashState::Ended) => write!(f, "Hasning / ended"),
            ModGetState::Ended => write!(f, "Hasning / ended"),
            ModGetState::Failed(FailedState::Hashing) => write!(f, "FAILED / hashing"),
            ModGetState::Failed(FailedState::Downloading) => write!(f, "FAILED / downloading"),
            ModGetState::Failed(FailedState::Unknown(s)) => write!(f, "FAILED / unknown {}", s),
        }
    }
}
