# turbobtainer
It doesn't just obtain videos; it Turbobtains them.
# 🚀 Turbobtainer

**Turbobtainer** is a high-performance, cross-platform desktop application designed for seamless multimedia extraction. By bridging the gap between a modern UI and industrial-grade CLI tools, Turbobtainer provides a streamlined experience for downloading video and audio from thousands of platforms.

[![Powered by Tauri](https://img.shields.io/badge/built%20with-Tauri-24c8db?style=flat-square&logo=tauri)](https://tauri.app/)
[![Angular](https://img.shields.io/badge/Frontend-Angular%2017-dd0031?style=flat-square&logo=angular)](https://angular.io/)
[![Rust](https://img.shields.io/badge/Backend-Rust-000000?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat-square)](https://opensource.org/licenses/MIT)

---

## 🛠️ Technology Stack

Turbobtainer leverages a robust "Deep Tech" stack to ensure memory safety, speed, and a native look-and-feel.

### **Core Architecture**
* **[Rust](https://www.rust-lang.org/):** The backbone of the application. Handles high-level systems logic, file I/O, and safe process management.
* **[Tauri v2](https://tauri.app/):** A security-focused framework that replaces heavy Chromium instances (Electron) with native WebViews, resulting in a tiny footprint (~10MB-20MB).

### **Frontend**
* **[Angular](https://angular.io/):** Provides a scalable, reactive architecture for the user interface.
* **[Ionic Framework](https://ionicframework.com/):** Used for polished UI components, ensuring a mobile-ready and accessible design.

### **Development Environment**
* **[Node.js](https://nodejs.org/):** Runtime for the frontend build pipeline.
* **[nvm (Node Version Manager)](https://github.com/nvm-sh/nvm):** Recommended for managing environment-specific Node versions to ensure build consistency.

### **Engines & Sidecars**
* **[yt-dlp](https://github.com/yt-dlp/yt-dlp):** A feature-rich command-line audio/video downloader (a fork of youtube-dl).
* **[FFmpeg](https://ffmpeg.org/):** The "Swiss Army Knife" of multimedia, used for muxing video and audio streams and encoding formats like MP3 and MP4.

### **Deployment & Distribution**
* **[WiX Toolset](https://wixtoolset.org/):** Utilized to generate professional Windows Installer (`.msi`) packages.
* **NSIS:** Used for creating lightweight, scriptable Windows setups (`.exe`).

---

## 🏗️ How It Works (The Sidecar Pattern)

Turbobtainer does not require users to install FFmpeg or yt-dlp manually. The application uses a **Sidecar Pattern**:
1.  **Detection:** On startup, the Rust backend detects the system's `target-triple` (e.g., `x86_64-pc-windows-msvc`).
2.  **Resolution:** It dynamically locates the bundled binaries within the application's internal resource directory.
3.  **Execution:** When a download is triggered, Rust spawns these binaries as child processes, piping real-time progress data back to the Angular frontend via Tauri Events.

---

## 🚀 Getting Started

### **Prerequisites**
* **Rust:** [Install Rustup](https://rustup.rs/)
* **Node.js:** v18+ (Use `nvm install 18` and `nvm use 18`)
* **Windows:** [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) and C++ Build Tools.

### **Setup & Installation**

1.  **Clone the repository:**
    ```bash
    git clone [https://github.com/your-username/turbobtainer.git](https://github.com/your-username/turbobtainer.git)
    cd turbobtainer
    ```

2.  **Install Frontend Dependencies:**
    ```bash
    npm install
    ```

3.  **Place Sidecar Binaries:**
    Create a `src-tauri/bin/` folder and place your platform-specific binaries. They must follow the naming convention:
    * `ffmpeg-x86_64-pc-windows-msvc.exe`
    * `yt-dlp-x86_64-pc-windows-msvc.exe`

4.  **Run in Development Mode:**
    ```bash
    npm run tauri dev
    ```

5.  **Build Production Installer:**
    ```bash
    npx ng build --configuration production
    npm run tauri build
    ```

---

## 📂 Project Structure

```text
turbobtainer/
├── src/                # Angular + Ionic Frontend
├── src-tauri/
│   ├── bin/            # Sidecar binaries (FFmpeg, yt-dlp)
│   ├── src/            # Rust Backend (lib.rs, main.rs)
│   ├── capabilities/   # Security and permission definitions
│   └── tauri.conf.json # Build and bundle configuration
├── dist/               # Compiled frontend assets
└── README.md
```

## ⚖️ Disclaimer

Turbobtainer is intended for personal use only. Please respect the terms of service of the websites you interact with. The developers of Turbobtainer are not responsible for any misuse of this tool or copyright infringement.

Created with ❤️ by Turbobtainer Team.