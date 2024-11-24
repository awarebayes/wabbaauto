use base64::prelude::*;
use std::fs::File;
use std::hash::Hasher;
use std::io::{BufReader, Read, Result};
use std::path::Path;
use tokio::sync::watch;
use twox_hash::XxHash64;

use crate::state::{HashState, ModGetState};

fn u64_to_base64(value: u64) -> String {
    let bytes = value.to_le_bytes();
    BASE64_STANDARD.encode(&bytes)
}

pub fn hash_file_xxhash64(
    path: &Path,
    progress_reporter: watch::Sender<ModGetState>,
) -> Result<String> {
    progress_reporter.send_replace(ModGetState::Hashing(HashState::Init));

    let file = File::open(path)?;
    let file_size = file.metadata().unwrap().len();
    let mut reader = BufReader::new(file);

    let mut hasher = XxHash64::with_seed(0);
    let mut buffer = [0; 8192];

    progress_reporter.send_replace(ModGetState::Hashing(HashState::Started));
    let mut total_bytes_read = 0;
    loop {
        let bytes_read = reader.read(&mut buffer)?;
        total_bytes_read += bytes_read;
        progress_reporter.send_replace(ModGetState::Hashing(HashState::InProgress {
            percent: (total_bytes_read as f64 / file_size as f64 * 100.0) as usize,
        }));
        if bytes_read == 0 {
            break;
        }
        hasher.write(&buffer[..bytes_read]);
    }

    progress_reporter.send_replace(ModGetState::Hashing(HashState::Ended));
    let u64_hash = hasher.finish();
    Ok(u64_to_base64(u64_hash))
}
