<img width="746" height="176" alt="banner" src="https://github.com/user-attachments/assets/12d94fdd-cb1d-4ff1-821e-cd97c24533c0" />

[![Tauri](https://img.shields.io/badge/built%20with-Tauri-24c8db?style=flat-square&logo=tauri)](https://tauri.app/)
[![Node.js](https://img.shields.io/badge/Runtime-Node.js-339933?style=flat-square&logo=node.js)](https://nodejs.org/)
[![Angular](https://img.shields.io/badge/Frontend-Angular%2017-dd0031?style=flat-square&logo=angular)](https://angular.io/)
[![Ionic](https://img.shields.io/badge/UI%20Framework-Ionic-3880ff?style=flat-square&logo=ionic)](https://ionicframework.com/)
[![Rust](https://img.shields.io/badge/Backend-Rust-000000?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![Engine: yt-dlp](https://img.shields.io/badge/Engine-yt--dlp-red?style=flat-square&logo=youtube)](https://github.com/yt-dlp/yt-dlp)
[![Muxer: FFmpeg](https://img.shields.io/badge/Muxer-FFmpeg-0078d7?style=flat-square&logo=ffmpeg)](https://ffmpeg.org/)

Turbobtainer is a high-performance desktop application designed for seamless multimedia extraction. By bridging the gap between a modern UI and industrial-grade CLI tools, Turbobtainer provides a streamlined experience for downloading video and audio specifically from [YouTube](https://www.youtube.com/), powered by the industry-standard [yt-dlp](https://github.com/yt-dlp/yt-dlp) engine and the versatile [FFmpeg](https://www.ffmpeg.org/) multimedia framework.

# Content
  
  - [Latest Release](#Latest-Release)
  - [Technology Stack](#Technology-Stack)
    - [**Core Architecture**](#Core-Architecture)
    - [**Frontend**](#Frontend)
    - [**Development Environment**](#Development-Environment)
    - [**Engines & Sidecars**](#Engines--Sidecars)
    - [**Deployment & Distribution**](#Deployment--Distribution)
  - [How It Works (The Sidecar Pattern)](#How-It-Works-The-Sidecar-Pattern)
  - [Getting Started](#Getting-Started)
    - [**Prerequisites**](#Prerequisites)
    - [**Setup & Installation**](#Setup--Installation)
  - [Project Structure](#Project-Structure)
  - [Disclaimer](#Disclaimer)
---

## Latest Release

| File | Description |
| :--- | :--- |
| `turbobtainer_1.0.0_x64.msi` | Windows Managed Installer (64-bit). Includes automated resource mapping and system integration. |

---

## Technology Stack

Turbobtainer leverages a robust "Deep Tech" stack to ensure memory safety, speed, and a native look-and-feel.

### **Core Architecture**
* **[Rust](https://www.rust-lang.org/):** The backbone of the application. Handles high-level systems logic, file I/O, and safe process management.
* **[Tauri v2](https://tauri.app/):** A security-focused framework that replaces heavy Chromium instances (Electron) with native WebViews, resulting in a tiny footprint (~10MB-20MB).

### **Frontend**
* **[Angular](https://angular.io/):** Provides a scalable, reactive architecture for the user interface.
* **[Ionic Framework](https://ionicframework.com/):** Used for polished UI components, ensuring a mobile-ready and accessible design.

### **Development Environment**
* **[Node.js](https://nodejs.org/):** Runtime for the frontend build pipeline.
* **[nvm (Node Version Manager)](https://github.com/nvm-sh/nvm):** Recommended for managing environment-specific Node versions.

### **Engines & Sidecars**
* **[yt-dlp](https://github.com/yt-dlp/yt-dlp):** A feature-rich command-line audio/video downloader.
* **[FFmpeg](https://ffmpeg.org/):** Used for muxing video/audio streams and encoding formats.

### **Deployment & Distribution**
* **[WiX Toolset](https://wixtoolset.org/):** Utilized to generate professional Windows Installer (`.msi`) packages.
* **NSIS:** Used for creating lightweight Windows setups (`.exe`).

---

## How It Works (The Sidecar Pattern)

Turbobtainer uses a **Sidecar Pattern** to execute external binaries without requiring manual installation from the user:

1.  **Detection:** On startup, the Rust backend detects the system's `target-triple`.
2.  **Resolution:** It dynamically locates the bundled binaries (`yt-dlp` and `ffmpeg`) within the application's internal resource directory.
3.  **Execution:** When a download starts, Rust spawns these binaries as child processes, piping real-time progress data back to the Angular frontend via Tauri Events.

---

## Getting Started

### **Prerequisites**
* **Rust:** [Install Rustup](https://rustup.rs/)
* **Node.js:** v18+ (Recommended via `nvm`)
* **Windows:** [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) and C++ Build Tools.
  
### **Setup & Installation**

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/1337KR3W/turbobtainer.git
    ```
    ```bash
    cd turbobtainer
    ```

2.  **Install Dependencies:**

    Angular CLI
    ```bash
    npm install -g @angular/cli
    ```
    Ionic CLI
    ```bash
    npm install -g @ionic/cli
    ```
    Tauri CLI
    ```bash
    npm install -g @tauri-apps/cli
    ```
    Git LFS
    ```bash
    git lfs install
    ```
    Git LFS
    ```bash
    git lfs track "src-tauri/bin/*.exe
    ```
    Packages
    ```bash
    npm install
    ```

4.  **Place Sidecar Binaries:**
    Place your platform-specific binaries in `src-tauri/bin/` following the naming convention:
    * `ffmpeg-x86_64-pc-windows-msvc.exe`
    * `yt-dlp-x86_64-pc-windows-msvc.exe`

5.  **Run & Build:**
    ```bash
    # Development
    npm run tauri dev

    # Production Build
    npx ng build --configuration production
    npm run tauri build
    ```

---

## Project Structure

```text
turbobtainer/
├── src/                # Angular + Ionic Frontend
├── src-tauri/
│   ├── bin/            # Sidecar binaries (FFmpeg, yt-dlp)
│   ├── src/            # Rust Backend (lib.rs, main.rs)
│   ├── capabilities/   # Security and permission definitions
│   └── tauri.conf.json # Build and bundle configuration
└── README.md
```

## Disclaimer

HOLD IT RIGHT THERE, TURBO-USER!

Turbobtainer is for saving your own memories or open-source content, not for building a pirate empire on your hard drive.

* Don’t be a pirate: Downloading copyrighted stuff without permission is uncool (and illegal).

* You’re the Captain: If you get into legal hot water, we’re jumping ship before you do.

* No Warranties: If this software becomes self-aware and decides to go on strike on Mondays, don't call us.

Use it wisely. If you agree, let the Turbobtainer-ing begin!

Turbobtainer by 1337KR3W.

---
