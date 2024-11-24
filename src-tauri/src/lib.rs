mod schema;
use chrono::Local;
use lib_wabbaauto::{
    check_nexus_login, download_modlist_status, test_downloader, AppState, DownloadManager,
};
use schema::DownloadReponse;
use tauri::{async_runtime::Mutex, Emitter, Manager, State};
use tokio::sync::watch;

#[derive(Debug, Default)]
struct AppData {
    downloading_modlist: Option<String>,
}

#[tauri::command]
async fn do_check_nexus_login() -> String {
    let login_status = check_nexus_login().await;
    let ret_status = serde_json::to_string(&login_status).unwrap();
    return ret_status;
}

struct DownloadHandler {
    app_handle: tauri::AppHandle,
    rx: watch::Receiver<AppState>,
}

impl DownloadHandler {
    fn new(app_handle: tauri::AppHandle, rx: watch::Receiver<AppState>) -> Self {
        Self { app_handle, rx }
    }

    async fn run(&mut self) {
        while self.rx.changed().await.is_ok() {
            let state = self.rx.borrow().clone();
            if let Err(e) = self
                .app_handle
                .emit("download-progress", serde_json::to_string(&state).unwrap())
            {
                eprintln!("Failed to emit event: {}", e);
            }
        }
    }
}

#[tauri::command]
async fn run_download_thread(
    app_handle: tauri::AppHandle,
    state: State<'_, Mutex<AppData>>,
    machine_url: &str,
    download_dir: &str,
) -> Result<String, ()> {
    let mut state = state.lock().await;
    if let Some(url) = &state.downloading_modlist {
        return Ok(serde_json::to_string(&DownloadReponse {
            error: format!(
                "Download is already in progress: {}. Restart the app or wait for it to complete",
                url
            ),
            success: false,
        })
        .unwrap());
    }

    let status = download_modlist_status(machine_url).await.unwrap();
    state.downloading_modlist = Some(machine_url.to_owned());

    let (tx, rx) = watch::channel(AppState::default());

    let download_dir = download_dir.to_owned();
    tokio::spawn(async move {
        let mut downloader = DownloadManager::new(status, download_dir);
        downloader.start(tx).await;
        // test_downloader(tx).await;
    });

    let mut handler = DownloadHandler::new(app_handle, rx);
    tokio::spawn(async move {
        handler.run().await;
    });

    Ok(serde_json::to_string(&DownloadReponse {
        error: String::default(),
        success: true,
    })
    .unwrap())
}

fn generate_log_filename() -> String {
    let now = Local::now();
    let formatted_time = now.format("%Y-%m-%d_%H-%M-%S").to_string();
    format!("{}_wabbaautodl_log.txt", formatted_time)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Folder {
                        path: std::path::PathBuf::from("./logs"),
                        file_name: Some(generate_log_filename()),
                    },
                ))
                .build(),
        )
        .manage(Mutex::new(AppData::default())) // Initialize and manage the state
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_websocket::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![do_check_nexus_login])
        .invoke_handler(tauri::generate_handler![run_download_thread])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
