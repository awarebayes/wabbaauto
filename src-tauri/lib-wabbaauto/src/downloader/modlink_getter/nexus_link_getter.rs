use async_trait::async_trait;
use tokio::sync::watch;

use crate::{downloader::{error::WabbaAutoError, headless::get_nexus_download_url}, state::ModGetState};

use super::modlink_getter::ModlinkGetter;

pub struct NexusModlinkGetter {
    game_name: String,
    mod_id: usize,
    file_id: usize,
}

#[async_trait]
impl ModlinkGetter for NexusModlinkGetter {
    async fn get_link(
        &self,
        progress_reporter: watch::Sender<ModGetState>,
    ) -> Result<String, WabbaAutoError> {
        get_nexus_download_url(
            &self.game_name,
            self.mod_id,
            self.file_id,
            progress_reporter.clone(),
        )
        .await
    }
}

impl NexusModlinkGetter {
    pub fn new(game_name: String, mod_id: usize, file_id: usize) -> Self {
        NexusModlinkGetter {
            game_name,
            mod_id,
            file_id,
        }
    }
}
