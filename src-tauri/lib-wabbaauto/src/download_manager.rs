use crate::downloader::{Downloader, NexusModlinkGetter};
use crate::schema::{ModOriginal, ModlistStatus};
use crate::state::{AppState, DownloadState, ModGetState};
use std::collections::HashMap;
use std::time::Duration;
use tokio;
use tokio::sync::watch;
use tokio::time::sleep;



pub struct DownloadManager {
    download_dir: String,
    mods_to_download: Vec<ModOriginal>,

    mods_downloading: Vec<ModOriginal>,
    mods_downloaded: Vec<ModOriginal>,
    mods_failed: Vec<ModOriginal>,
    mods_failed_repr: Vec<String>,
    total_mods: usize,
    downloads_in_progress: HashMap<String, Downloader>,
    n_concurrent_downloads: usize,
}

impl DownloadManager {
    pub fn new(status: ModlistStatus, download_dir: String) -> Self {
        let nexus_archives = status
            .archives
            .iter()
            .filter_map(|i| {
                if i.original.state.type_field.contains("NexusDownloader") {
                    return Some(i.original.clone());
                }
                None
            })
            .collect::<Vec<_>>();

        let total_mods =  nexus_archives.len();

        return DownloadManager {
            mods_to_download: nexus_archives,
            mods_downloaded: Vec::new(),
            mods_downloading: Vec::new(),
            mods_failed_repr: Vec::new(),
            mods_failed: Vec::new(),
            downloads_in_progress: HashMap::new(),
            n_concurrent_downloads: 8,
            total_mods,
            download_dir,
        };
    }

    pub async fn start(&mut self, sender: watch::Sender<AppState>) {
        while self.mods_to_download.len() > 0 || self.downloads_in_progress.len() > 0 {
            while !self.mods_to_download.is_empty()
                && self.downloads_in_progress.len() < self.n_concurrent_downloads
            {
                let game_mod = self.mods_to_download.pop().unwrap();
                self.mods_downloading.push(game_mod.clone());

                let nexus_modlink_getter = NexusModlinkGetter::new(
                    game_mod.state.game_name.unwrap(),
                    game_mod.state.mod_id.unwrap(),
                    game_mod.state.file_id.unwrap(),
                );

                let download = Downloader::new(
                    game_mod.name,
                    self.download_dir.clone(),
                    game_mod.hash.clone(),
                    nexus_modlink_getter,
                );
                self.downloads_in_progress
                    .insert(game_mod.state.name.unwrap(), download);
            }

            self.check_completed_downloads();
            self.check_failed_downloads();

            let state = self.get_state();
            sender.send_replace(state);
            sleep(Duration::from_millis(500)).await;
        }
    }

    fn check_completed_downloads(&mut self) {
        let completed_downloads = self
            .downloads_in_progress
            .iter()
            .filter_map(|(modname, handle)| {
                if handle.get_state().has_ended() {
                    return Some(modname);
                }
                None
            })
            .cloned()
            .collect::<Vec<_>>();

        self.mods_downloaded.extend(
            self.mods_downloading
                .iter()
                .filter(|game_mod| {
                    completed_downloads.contains(game_mod.state.name.as_ref().unwrap())
                })
                .cloned(),
        );

        self.mods_downloading = self
            .mods_downloading
            .iter()
            .filter(|game_mod| !completed_downloads.contains(game_mod.state.name.as_ref().unwrap()))
            .cloned()
            .collect();

        for i in completed_downloads {
            self.downloads_in_progress.remove(&i).unwrap();
        }
    }

    fn check_failed_downloads(&mut self) {
        let failed_downloads = self
            .downloads_in_progress
            .iter()
            .filter_map(|(modname, handle)| {
                if handle.get_state().has_failed() {
                    return Some(modname);
                }
                None
            })
            .cloned()
            .collect::<Vec<_>>();

        let failed_downloads_repr = self
            .downloads_in_progress
            .iter()
            .filter_map(|(modname, handle)| {
                if handle.get_state().has_failed() {
                    return Some(format!("{} {}", &modname, handle.get_state()));
                }
                None
            })
            .collect::<Vec<_>>();

        self.mods_failed.extend(
            self.mods_downloading
                .iter()
                .filter(|game_mod| {
                    failed_downloads.contains(game_mod.state.name.as_ref().unwrap())
                })
                .cloned(),
        );

        self.mods_downloading = self
            .mods_downloading
            .iter()
            .filter(|game_mod| !failed_downloads.contains(game_mod.state.name.as_ref().unwrap()))
            .cloned()
            .collect();

        for i in failed_downloads {
            self.downloads_in_progress.remove(&i).unwrap();
        }

        self.mods_failed_repr.extend(failed_downloads_repr);
    }

    fn get_state(&self) -> AppState {
        let mut downloading = self
            .downloads_in_progress
            .iter()
            .map(|(name, handle)| (name.clone(), handle.get_state()))
            .collect::<Vec<_>>();
        downloading.sort_by(|a, b| a.0.as_str().partial_cmp(b.0.as_str()).unwrap());
        let recent_fails = self.mods_failed_repr.iter().rev().take(50).cloned().collect::<Vec<_>>();

        AppState {
            getting_archives: downloading,
            failed: self.mods_failed.len(),
            successes: self.mods_downloaded.len(),
            total: self.total_mods,
            recent_fails,
        }
    }
}

pub async fn test_downloader(sender: watch::Sender<AppState>) {
    loop {
        let mut downloading = AppState::default();
        downloading.failed = 1234;
        downloading.successes = 4321;
        let mod_download_state = (0..8)
            .map(|x| {
                (
                    "Mod ".to_string() + x.to_string().as_str(),
                    ModGetState::Downloading(DownloadState::InProgress {
                        percent: fastrand::usize(0..100),
                        speed_kbps: fastrand::usize(0..1000),
                        downloaded: fastrand::usize(0..100000),
                        total_size: fastrand::usize(0..10000000),
                    }),
                )
            })
            .collect::<Vec<_>>();
        downloading.getting_archives = mod_download_state;
        downloading.recent_fails = vec!["A - 1234".into(), "B - 4512".into(), "C - 3453".into()];
        sender.send_replace(downloading);
        sleep(Duration::from_millis(500)).await;
    }
}
