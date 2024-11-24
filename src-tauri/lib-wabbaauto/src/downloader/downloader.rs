use super::{error::WabbaAutoError, modlink_getter::ModlinkGetter};
use crate::{
    downloader::util::{download_file, hash_file_xxhash64},
    state::{FailedState, ModGetState},
};
use std::{fs, path::Path};
use tokio::{sync::watch, task::JoinHandle};

pub struct Downloader {
    join_handle: JoinHandle<Result<(), WabbaAutoError>>,
    _rx: watch::Receiver<ModGetState>,
}

async fn downloader_thread(
    filename: String,
    download_dir: String,
    link_getter: impl ModlinkGetter,
    hash: String,
    tx: watch::Sender<ModGetState>,
) -> Result<(), WabbaAutoError> {
    let maybe_download_file = Path::new(download_dir.as_str()).join(filename.as_str());
    if fs::exists(&maybe_download_file).unwrap() {
        log::info!("File already downloaded {}, hashing", filename);
        let calculated_hash = hash_file_xxhash64(&maybe_download_file, tx.clone()).map_err(|_| WabbaAutoError::HashingError)?;
        if hash != calculated_hash {
            log::warn!(
                "Hashing failed, expected {} got {}, we will try downloading and hashing again",
                hash,
                calculated_hash
            );
        } else {
            tx.send_replace(ModGetState::Ended);
            return Ok(());
        }
    }
    log::info!("Getting file {}", filename.clone());
    let url = link_getter.get_link(tx.clone()).await?;
    log::info!("Going to download a file {}", url);
    let downloaded_file = download_file(url, filename, download_dir, tx.clone()).await?;
    let calculated_hash = hash_file_xxhash64(&downloaded_file, tx.clone()).map_err(|_| WabbaAutoError::HashingError)?;
    if hash != calculated_hash {
        tx.send_replace(ModGetState::Failed(FailedState::Hashing));
        return Err(WabbaAutoError::HashMismatchError)
    }
    tx.send_replace(ModGetState::Ended);

    Ok(())
}

impl Downloader {
    pub fn new(
        filename: String,
        download_dir: String,
        hash: String,
        link_getter: impl ModlinkGetter + Send + 'static,
    ) -> Self {
        let (tx, rx) = watch::channel(ModGetState::Init);
        let join_handle = tokio::spawn(async move {
            let download_res =
                downloader_thread(filename, download_dir, link_getter, hash, tx.clone()).await;
            match download_res {
                Ok(_) => (),
                Err(err) => {
                    tx.send_replace(ModGetState::Failed(FailedState::Unknown(err.to_string())));
                }
            }
            Ok(())
        });
        Downloader {
            join_handle,
            _rx: rx,
        }
    }

    pub fn get_state(&self) -> ModGetState {
        self._rx.borrow().clone()
    }

    pub fn abort(&mut self) {
        self.join_handle.abort();
    }
}
