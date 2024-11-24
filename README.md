# 🚀 Wabba Auto Downloader

[![Chrome Automation](https://img.shields.io/badge/Chrome-Automation-blue)](https://www.chromium.org/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://opensource.org/licenses/MIT)

Automate your Nexus mod downloads using a botted Google Chrome instance. Supports both interactive and headless operation modes.

## ✨ Features

- 🤖 Automated mod downloads from NexusMods (LL, others planned)
- 🎯 Support for wabbajack modlists
- 📦 Works on top of portable chrome
- 👻 Optional headless mode operation
- 📦 Single self contained binary
- 🖥️ Optional [TUI](https://github.com/awarebayes/wabbaauto/tree/release/src-tauri/cli) w [RataTui](https://ratatui.rs/)



## 🚀 Quick Start Guide

### 1. Set Up Chrome Portable

1. Download Chrome Portable from [PortableApps.com](https://portableapps.com/apps/internet/google_chrome_portable)
   > 🔒 Is PortableApps safe? Check their [Wikipedia page](https://en.wikipedia.org/wiki/PortableApps.com)
2. Install to your preferred location (e.g., Downloads folder)

### 2. Initial Configuration

1. Launch the installed portable Chrome
2. Navigate to [NexusMods](https://www.nexusmods.com)
3. Log into your account
4. Close portable Chrome

### 3. Launch with Debugger

Open PowerShell and run:

```powershell
# For interactive mode
Start-Process "C:\Users\[YourUsername]\Downloads\Chrome\GoogleChromePortable\GoogleChromePortable.exe" -ArgumentList "--remote-debugging-port=9222"

# For headless mode (optional)
Start-Process "C:\Users\[YourUsername]\Downloads\Chrome\GoogleChromePortable\GoogleChromePortable.exe" -ArgumentList "--remote-debugging-port=9222 --headless"
```

### 4. Start Downloading

1. Launch Wabba Auto Downloader
2. Select your modlist
3. Begin the download process
4. Verify everything works as expected

## 💡 Pro Tips

- Use headless mode for background operation

## 🤝 Contributing

Feel free to:
- Open issues for bugs
- Submit feature requests
- Create pull requests

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---
Made with ❤️ for the modding community