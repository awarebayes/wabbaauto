# 🚀 Wabba Auto Downloader

[![Chrome Automation](https://img.shields.io/badge/Chrome-Automation-blue)](https://www.chromium.org/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://opensource.org/licenses/MIT)

Automate your Nexus mod downloads using a botted Google Chrome instance.

The development of this tool stops, when the Nexus Mods team adds crypto payment option, so that everyone is able to pay for the mods.

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

![Modlists](https://i.ibb.co/ChmSLxV/389278968-8e5e9729-bb5d-46a4-9a3c-4adac2cbc3e6.png)
![Downloading](https://i.ibb.co/6BRm3ND/389278785-1364be2c-5ac6-4ac6-a18f-289bb40f4f0c.png)

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
