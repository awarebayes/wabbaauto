use crate::downloader::error::WabbaAutoError;
use crate::state::{DownloadState, ModGetState};
use crate::ModlistStatus;
use futures_lite::stream::StreamExt;
use log;
use reqwest::Client;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Instant;
use tokio::sync::watch;

fn median(values: Vec<usize>) -> usize {
    let mut values_copy = values;
    values_copy.sort();
    return values_copy[values_copy.len() / 2];
}

pub async fn download_file(
    url: String,
    filename: String,
    download_dir: String,
    progress_reporter: watch::Sender<ModGetState>,
) -> Result<PathBuf, WabbaAutoError> {
    let download_dir = Path::new(download_dir.as_str());
    let original_filename = Path::new(filename.as_str());
    let download_filename_str = filename.clone() + ".download";
    let download_filename = Path::new(download_filename_str.as_str());
    let original_file = download_dir.join(original_filename);
    let download_file = download_dir.join(download_filename);
    log::info!(
        "Created a download file: {}",
        download_file.clone().to_str().unwrap()
    );

    progress_reporter.send(ModGetState::Downloading(DownloadState::Started)).unwrap();

    let mut file = File::create(download_file.clone()).unwrap();

    let client = Client::new();
    let response = client.get(url).send().await.map_err(|_| WabbaAutoError::GetFailed)?;

    if !response.status().is_success() {
        return Err(WabbaAutoError::GetFailed);
    }

    let total_size = response.content_length().unwrap_or(0);
    let mut current_percent;
    let mut downloaded: u64 = 0;

    const SPEED_FREQ: usize = 10;
    let mut stream = response.bytes_stream();
    let mut last_time = Instant::now();
    let mut average_speed = vec![0; SPEED_FREQ];
    let mut i = 0;
    let mut speed;

    while let Some(chunk) = stream.next().await {
        let data = chunk.unwrap();
        downloaded += data.len() as u64;

        let chunk_size = data.len() as u64;
        current_percent = (downloaded as f64 / total_size as f64 * 100.0) as usize;

        let elapsed = last_time.elapsed().as_secs_f64();
        let speed_kbps = ((chunk_size as f64 / 1024.0) / elapsed) as usize;
        average_speed[i % SPEED_FREQ] = speed_kbps;
        last_time = Instant::now();

        if i % SPEED_FREQ == 0 {
            speed = median(average_speed.to_vec());
            progress_reporter.send_replace(ModGetState::Downloading(DownloadState::InProgress {
                percent: current_percent,
                speed_kbps: speed,
                downloaded: downloaded as usize,
                total_size: total_size as usize,
            }));
        }

        file.write_all(&data).unwrap();
        i += 1;
    }

    fs::rename(download_file.clone(), original_file.clone()).unwrap();
    log::info!(
        "Downloaded successfully: {}",
        original_file.to_str().unwrap()
    );

    progress_reporter.send(ModGetState::Downloading(DownloadState::Ended)).unwrap();
    Ok(original_file)
}


pub async fn download_modlist_status(machine_url: &str) -> Result<ModlistStatus, WabbaAutoError> {
    log::debug!("Getting modlist status");
    let modlist_status = reqwest::get(
        format!("https://raw.githubusercontent.com/wabbajack-tools/mod-lists/refs/heads/master/reports/{}/status.json", machine_url)
    ).await
    .map_err(|_| WabbaAutoError::GetFailed)?
    .json::<ModlistStatus>()
    .await
    .unwrap();
    Ok(modlist_status)
}