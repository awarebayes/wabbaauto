use thiserror::Error;

#[derive(Error, Debug)]
pub enum WabbaAutoError {
    #[error("Chrome is not running, could not connect to localhost:9222")]
    ChromeIsNotRunningError,
    #[error("Nexus login error")]
    NexusLoginError,
    #[error("Getting link error")]
    GettingLinkError,
    #[error("Hash mismatch occured")]
    HashMismatchError,
    #[error("GET failed")]
    GetFailed,
    #[error("Headless Chrome error")]
    HeadlessChromeError,
    #[error("Other error")]
    OtherError(String),
    #[error("Hashing error")]
    HashingError,
}