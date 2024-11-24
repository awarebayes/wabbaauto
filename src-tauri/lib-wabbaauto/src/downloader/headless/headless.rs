use headless_chrome::protocol::cdp::Page;
use headless_chrome::protocol::cdp::Target::CreateTarget;
use headless_chrome::Browser;
use reqwest::get;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::watch;
use tokio::sync::Mutex;

use super::js_injection;
use crate::downloader::error::WabbaAutoError;
use crate::schema::LoginStatus;
use crate::state::{GettingLinkState, ModGetState};
use once_cell::sync::Lazy;

static CHROME_DEBUG_URL: &str = "http://localhost:9222/json/version";
static SINGLE_GET_LINK_MUTEX: Lazy<Arc<Mutex<()>>> = Lazy::new(|| Arc::new(Mutex::new(())));

pub async fn connect_to_browser() -> Result<Browser, WabbaAutoError> {
    let json = get(CHROME_DEBUG_URL)
        .await
        .map_err(|_| WabbaAutoError::ChromeIsNotRunningError)?
        .json::<HashMap<String, String>>()
        .await
        .map_err(|_| WabbaAutoError::OtherError("JSON returned by chrome is not valid".into()))?;

    let debug_ws_url = json
        .get("webSocketDebuggerUrl")
        .expect("No field webSocketDebuggerUrl was found in localhost:9222 json");

    let browser =
        Browser::connect(debug_ws_url.clone()).map_err(|_| WabbaAutoError::HeadlessChromeError)?;
    Ok(browser)
}

pub async fn nexus_login() -> Result<bool, WabbaAutoError> {
    let browser = connect_to_browser().await?;
    let tab = browser
        .new_tab()
        .map_err(|_| WabbaAutoError::HeadlessChromeError)?;

    tab.navigate_to("http://nexusmods.com")
        .map_err(|_| WabbaAutoError::HeadlessChromeError)?;
    tab.wait_for_element("header#head")
        .map_err(|_| WabbaAutoError::HeadlessChromeError)?;

    let is_logged_in = tab.find_element("a#login").is_err();
    Ok(is_logged_in)
}

pub async fn get_nexus_download_url(
    game_name: &str,
    mod_id: usize,
    file_id: usize,
    progress_reporter: watch::Sender<ModGetState>,
) -> Result<String, WabbaAutoError> {
    let _guard = SINGLE_GET_LINK_MUTEX.lock().await;

    progress_reporter.send_replace(ModGetState::GettingLink(GettingLinkState::Init));
    let browser = connect_to_browser().await?;
    let url = format!(
        "https://www.nexusmods.com/{}/mods/{}?tab=files&file_id={}",
        game_name.to_lowercase(),
        mod_id,
        file_id
    );

    log::info!("Getting mod at link {}", url);

    progress_reporter.send_replace(ModGetState::GettingLink(
        GettingLinkState::ConnectedToBrowser,
    ));

    let tab = browser
        .new_tab_with_options(CreateTarget {
            url: url.clone(),
            background: Some(false),
            width: Some(1920),
            height: Some(1080),
            browser_context_id: None,
            enable_begin_frame_control: None,
            new_window: None,
        })
        .expect("Could not open a new tab after connecting to chrome");

    // let jpeg_data = tab.capture_screenshot(
    //     Page::CaptureScreenshotFormatOption::Jpeg,
    //     None,
    //     None,
    //     true).unwrap();
    // std::fs::write("screenshot.jpeg", jpeg_data).unwrap();
    tab.wait_for_element("button#slowDownloadButton").map_err(|_| WabbaAutoError::HeadlessChromeError)?;

    tab.evaluate(format!("window.file_id = {}", file_id).as_str(), true)
        .map_err(|_| WabbaAutoError::HeadlessChromeError)?;
    tab.evaluate(js_injection::JS_INJECTION, true)
        .map_err(|_| WabbaAutoError::HeadlessChromeError)?;
    tab.wait_for_element("button#slowDownloadButton")
        .map_err(|_| WabbaAutoError::HeadlessChromeError)?
        .click()
        .map_err(|_| WabbaAutoError::HeadlessChromeError)?;
    progress_reporter.send_replace(ModGetState::GettingLink(GettingLinkState::WaitingForLink));

    let a = tab
        .wait_for_element("a#downloadUrl")
        .map_err(|_| WabbaAutoError::HeadlessChromeError)?;
    let href = a
        .get_attribute_value("href")
        .map_err(|_| WabbaAutoError::HeadlessChromeError)?;
    tab.close(true)
        .map_err(|_| WabbaAutoError::HeadlessChromeError)?;

    progress_reporter.send_replace(ModGetState::GettingLink(GettingLinkState::Ended));

    return match href {
        Some(url) => Ok(url),
        None => Err(WabbaAutoError::HeadlessChromeError),
    };
}

pub async fn check_nexus_login() -> LoginStatus {
    let chrome_is_running = connect_to_browser().await;
    if chrome_is_running.is_err() {
        return LoginStatus {
            chrome: false,
            website: false,
            error: chrome_is_running
                .err()
                .map(|x| x.to_string())
                .unwrap_or_default(),
        };
    }
    let nexus_login_status = nexus_login().await;
    LoginStatus {
        chrome: true,
        website: nexus_login_status.is_ok(),
        error: nexus_login_status
            .err()
            .map(|x| x.to_string())
            .unwrap_or_default(),
    }
}
