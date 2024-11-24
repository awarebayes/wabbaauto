use async_trait::async_trait;
use tokio::sync::watch;

use crate::{downloader::error::WabbaAutoError, state::ModGetState};

#[async_trait]
pub trait ModlinkGetter {
    async fn get_link(&self, progress_reporter: watch::Sender<ModGetState>) -> Result<String, WabbaAutoError>;
}
