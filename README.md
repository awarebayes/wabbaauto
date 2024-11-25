# 🚀 Wabba Auto Downloader

[![Chrome Automation](https://img.shields.io/badge/Chrome-Automation-blue)](https://www.chromium.org/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://opensource.org/licenses/MIT)

Automate your Nexus mod downloads using a botted Google Chrome instance. Supports both interactive and headless operation modes.

## ✨ Features

- 🦀 Blazingly fast 🔥🔥🔥
- 🌐 Cross Platform (Windows, Linux, MacOS)
- 🤖 Automated mod downloads from NexusMods
- 🎯 Support for wabbajack modlists
- 📦 Works on top of portable chrome
- 👻 Optional headless mode operation
- 📦 Single self contained binary (installer on windows)
- 🖥️ Optional [TUI](https://github.com/awarebayes/wabbaauto/tree/release/src-tauri/cli) w [RataTui](https://ratatui.rs/)
- 🔒 Hashsum support checking

Planned:

- ❤️ Loverslab support
- 🔄 Round Robin download queue
- 💦 Shadcn UI

![Modlists](https://private-user-images.githubusercontent.com/42784580/389278785-1364be2c-5ac6-4ac6-a18f-289bb40f4f0c.png?jwt=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3MzI1MjkwODAsIm5iZiI6MTczMjUyODc4MCwicGF0aCI6Ii80Mjc4NDU4MC8zODkyNzg3ODUtMTM2NGJlMmMtNWFjNi00YWM2LWExOGYtMjg5YmI0MGY0ZjBjLnBuZz9YLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFWQ09EWUxTQTUzUFFLNFpBJTJGMjAyNDExMjUlMkZ1cy1lYXN0LTElMkZzMyUyRmF3czRfcmVxdWVzdCZYLUFtei1EYXRlPTIwMjQxMTI1VDA5NTk0MFomWC1BbXotRXhwaXJlcz0zMDAmWC1BbXotU2lnbmF0dXJlPTA5ZDliNjUxY2ZkODRmYWI0MDc2Njc2NDQ5N2JmZGYwYjZmN2YxMzY1OWQwYmEzZWY0Y2UxMTYxN2JkMThhMDcmWC1BbXotU2lnbmVkSGVhZGVycz1ob3N0In0.Mgijp9mOQdDvJy7ubWX2nKo1a16XB1PRK8_Ai3X3x8Y)
![Downloading](https://private-user-images.githubusercontent.com/42784580/389278968-8e5e9729-bb5d-46a4-9a3c-4adac2cbc3e6.png?jwt=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3MzI1MjkwODAsIm5iZiI6MTczMjUyODc4MCwicGF0aCI6Ii80Mjc4NDU4MC8zODkyNzg5NjgtOGU1ZTk3MjktYmI1ZC00NmE0LTlhM2MtNGFkYWMyY2JjM2U2LnBuZz9YLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFWQ09EWUxTQTUzUFFLNFpBJTJGMjAyNDExMjUlMkZ1cy1lYXN0LTElMkZzMyUyRmF3czRfcmVxdWVzdCZYLUFtei1EYXRlPTIwMjQxMTI1VDA5NTk0MFomWC1BbXotRXhwaXJlcz0zMDAmWC1BbXotU2lnbmF0dXJlPWJhZWQ2YWZjNzZlMTRhYzBiNmY5NGQzNDgxZThjYmVjZGZhMGEyNTRiYmVkZGViN2JlZjk4OTcwOTY5YWJmNWUmWC1BbXotU2lnbmVkSGVhZGVycz1ob3N0In0.tqKj9y2HA1lnOpAPpmtm-5VcNF0-RVJ_y12rupgGwlk)

## 🚀 Quick Start Guide

To grab the executable, go get the [latest release](https://github.com/awarebayes/wabbaauto/releases) for your platform.

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

Adjust path to match your installation dir if needed.

Visit `http://localhost:9222/json/version` in the other browser to ensure chrome is working.
If this page does not load, something went wrong with setting up google chrome.

### 4. Start Downloading

1. Launch Wabba Auto Downloader
2. Select your modlist
3. Begin the download process
4. Verify everything works as expected

### 5. Run Wabbajack

1. Launch wabbajack
2. Select downloaded modlist
3. Choose wabbajack downloads folder to match WabbaAuto downloads folder
4. Wait for wabbajack to complete downloading other files and installing

## 💡 Pro Tips

- Use headless mode for background operation
- Use Non-Headless mode for debugging why something does not work

## 🤝 Contributing

Feel free to:

- Open issues for bugs
- Submit feature requests
- Create pull requests

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

Made with ❤️ for the modding community
