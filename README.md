# 🚀 Turbobtainer

It doesnt just obtain videos; it Turbobtains them.

**Turbobtainer** is a high-performance desktop application designed for seamless multimedia extraction. By bridging the gap between a modern UI and industrial-grade CLI tools, Turbobtainer provides a streamlined experience for downloading video and audio from thousands of platforms.

## 📋 Table of Contents
1. [Latest Release](#-latest-release)
2. [Technology Stack](#-technology-stack)
3. [How It Works (Sidecar Pattern)](#-how-it-works-the-sidecar-pattern)
4. [Getting Started](#-getting-started)
5. [Project Structure](#-project-structure)
6. [Disclaimer](#-disclaimer)

---

## 📦 Latest Release

| File | Description |
| :--- | :--- |
| `turbobtainer_1.0.0_x64.msi` | Windows Managed Installer (64-bit). Includes automated resource mapping and system integration. |

---

[![Powered by Tauri](https://img.shields.io/badge/built%20with-Tauri-24c8db?style=flat-square&logo=tauri)](https://tauri.app/)
[![Angular](https://img.shields.io/badge/Frontend-Angular%2017-dd0031?style=flat-square&logo=angular)](https://angular.io/)
[![Rust](https://img.shields.io/badge/Backend-Rust-000000?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat-square)](https://opensource.org/licenses/MIT)

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
* **[nvm (Node Version Manager)](https://github.com/nvm-sh/nvm):** Used for managing environment-specific Node versions to ensure build consistency.

### **Engines & Sidecars**
* **[yt-dlp](https://github.com/yt-dlp/yt-dlp):** A feature-rich command-line audio/video downloader.
* **[FFmpeg](https://ffmpeg.org/):** Used for muxing video/audio streams and encoding formats.

### **Deployment & Distribution**
* **[WiX Toolset](https://wixtoolset.org/):** Utilized to generate professional Windows Installer (`.msi`) packages.
* **NSIS:** Used for creating lightweight Windows setups (`.exe`).

---

## 🏗️ How It Works (The Sidecar Pattern)

Turbobtainer uses a **Sidecar Pattern** to execute external binaries without requiring manual installation from the user:
1.  **Detection:** On startup, the Rust backend detects the system's `target-triple`.
2.  **Resolution:** It dynamically locates the bundled binaries (`yt-dlp` and `ffmpeg`) within the application's internal resource directory.
3.  **Execution:** When a download starts, Rust spawns these binaries as child processes, piping real-time progress data back to the Angular frontend via Tauri Events.

---

## 🚀 Getting Started

### **Prerequisites**
* **Rust:** [Install Rustup](https://rustup.rs/)
* **Node.js:** v18+ (Recommended via `nvm`)
* **Windows:** [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) and C++ Build Tools.

### **Setup & Installation**

1.  **Clone the repository:**
    ```bash
    git clone [https://github.com/your-username/turbobtainer.git](https://github.com/your-username/turbobtainer.git)
    cd turbobtainer
    ```

2.  **Install Dependencies:**
    ```bash
    npm install
    ```

3.  **Place Sidecar Binaries:**
    Place your platform-specific binaries in `src-tauri/bin/` following the naming convention:
    * `ffmpeg-x86_64-pc-windows-msvc.exe`
    * `yt-dlp-x86_64-pc-windows-msvc.exe`

4.  **Run & Build:**
    ```bash
    # Development
    npm run tauri dev

    # Production Build
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
└── README.md