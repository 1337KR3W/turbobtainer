use std::path::PathBuf;
use std::sync::Mutex;
use tauri_plugin_shell::process::{CommandEvent, CommandChild}; 
use tauri_plugin_shell::ShellExt;
use tauri::{AppHandle, Manager, Emitter};
#[derive(serde::Serialize)]
pub struct VideoMetadata {
    pub title: String,
    pub thumbnail: String,
    pub duration: String,
    pub size: String,
    pub has_playlist: bool
}

#[derive(serde::Deserialize)]
struct YtDlpOutput {
    title: Option<String>,
    thumbnail: Option<String>,
    duration_string: Option<String>,
    filesize: Option<f64>,
    filesize_approx: Option<f64>
}

pub struct DownloadState {
    // Cambiado Child -> CommandChild
    pub child: Mutex<Option<CommandChild>>, 
}

// Inicialización por defecto para el main.rs
impl Default for DownloadState {
    fn default() -> Self {
        Self {
            child: Mutex::new(None),
        }
    }
}

fn get_ffmpeg_path(app: &AppHandle) -> Result<PathBuf, String> {
    let target_triple = tauri::utils::platform::target_triple().unwrap_or_default();
    let ffmpeg_name = format!("ffmpeg-{}.exe", target_triple);

    // CASE A: DEVELOP (debug) ---
    #[cfg(debug_assertions)]
    {
        let dev_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("bin")
            .join(&ffmpeg_name);
        
        if dev_path.exists() {
            return Ok(dev_path);
        }
    }

    // CASE B: PRODUCTION (release) ---
    #[cfg(not(debug_assertions))]
    {
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                let prod_path = exe_dir.join(&ffmpeg_name);
                if prod_path.exists() { return Ok(prod_path); }
            }
        }
        let res_path = app.path().resource_dir()
            .unwrap_or_default()
            .join("_up_")
            .join("bin")
            .join(&ffmpeg_name);
            
        if res_path.exists() { return Ok(res_path); }
    }

    Err(format!("FFmpeg not found. Expected file: {}", ffmpeg_name))
}

#[tauri::command]
pub async fn check_video_url(app: AppHandle, url: String) -> Result<VideoMetadata, String> {
    if url.trim().is_empty() { return Err("URL empty".into()); }
    let has_playlist = url.contains("youtube.com") && url.contains("list=");
    let output = app.shell().sidecar("yt-dlp")
        .map_err(|e| format!("Engine error: {}", e))?
        .args([
            "--quiet",
            "--no-warnings",
            "--no-playlist",
            "--skip-download",
            "--dump-json",
            &url
        ])
        .output().await
        .map_err(|e| format!("Execution error: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Deserializamos el dump completo usando tu struct YtDlpOutput
    // Serde ignorará todos los campos de yt-dlp que no estén en tu struct.
    let raw: YtDlpOutput = serde_json::from_str(&stdout)
        .map_err(|e| format!("JSON Error: {} | Content: {}", e, stdout))?;

    let size_bytes = raw.filesize.or(raw.filesize_approx).unwrap_or(0.0);
    
    Ok(VideoMetadata {
        title: raw.title.unwrap_or_else(|| "Unknown Title".into()),
        thumbnail: raw.thumbnail.unwrap_or_default(),
        duration: raw.duration_string.unwrap_or_else(|| "00:00".into()),
        size: format!("{:.2} MB", size_bytes / 1048576.0),
        has_playlist,
    })
}

#[tauri::command]
pub async fn download_video(
    app: AppHandle,
    state: tauri::State<'_, DownloadState>,
    url: String,
    stype: String,
    download_playlist: bool,
) -> Result<String, String> {
    let ffmpeg_path = get_ffmpeg_path(&app)?;
    let download_dir = app.path().download_dir().map_err(|e| e.to_string())?;

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    
    let folder_name = format!("Turbobtainer_{}", timestamp);

    let output_tmpl = if download_playlist {
        download_dir
            .join(&folder_name)
            .join("%(playlist_title).100s")
            .join("%(title).100s.%(ext)s")
    } else {
        download_dir.join(format!("%(title).150s_{}.%(ext)s", folder_name))
    };

    let output_str = output_tmpl.to_string_lossy().to_string();

    let mut args = vec![
        "--newline",
        "--progress",
        "--progress-template", "PROG:%(progress._percent_str)s | TITLE:%(info.title)s",
        "-o", &output_str,
        "--ffmpeg-location", ffmpeg_path.to_str().ok_or("Invalid path")?,
        "--restrict-filenames",
    ];

    if !download_playlist {
        args.push("--no-playlist");
    } else {
        args.push("--yes-playlist");
    }

    if stype == "audio" {
        args.extend(["-x", "--audio-format", "mp3", "--audio-quality", "0"]);
    } else {
        args.extend(["-f", "bv*[ext=mp4]+ba[ext=m4a]/b[ext=mp4] / bv*+ba/b", "--merge-output-format", "mp4"]);
    }
    
    args.push(&url);

    // 1. Cambiamos _child por child (sin el guion bajo) para poder usarlo
    let (mut rx, child) = app.shell().sidecar("yt-dlp")
        .map_err(|e| e.to_string())?
        .args(args)
        .spawn()
        .map_err(|e| e.to_string())?;
    
    // 2. Guardamos el proceso hijo en el estado global para que STOP pueda matarlo
    {
        let mut lock = state.child.lock().unwrap();
        *lock = Some(child); 
    }
    
    // 3. Manejo de eventos en segundo plano
    tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stdout(line) => {
                    let out = String::from_utf8_lossy(&line);
    
                    if out.contains("Downloading playlist:") {
                        if let Some(playlist_name) = out.split("Downloading playlist: ").last() {
                            let _ = app.emit("playlist-title", playlist_name.trim());
                        }
                    }
    
                    if let Some(pos_prog) = out.find("PROG:") {
                        let raw_pct = out[pos_prog + 5..].split('|').next().unwrap_or("").trim().trim_end_matches('%');
                        if let Ok(pct) = raw_pct.parse::<f32>() {
                            let _ = app.emit("download-progress", pct / 100.0);
                        }
                    }
    
                    if let Some(pos_title) = out.find("TITLE:") {
                        let title = out[pos_title + 6..].trim();
                        if !title.is_empty() {
                            let _ = app.emit("item-title", title);
                        }
                    }
                },
                CommandEvent::Terminated(payload) => {
                    if payload.code == Some(0) {
                        let _ = app.emit("download-finished", true);
                    }
                },
                _ => {}
            }
        }
    });

    Ok("Download started".into())
}

#[tauri::command]
pub async fn stop_download(state: tauri::State<'_, DownloadState>) -> Result<(), String> {

    let mut lock = state.child.lock().unwrap();
    
    if let Some(mut child) = lock.take() { 
        child.kill().map_err(|e| format!("Error stopping process: {}", e))?;
        println!("Exit aborting process.");
    } else {
        println!("Non active process to be stopped.");
    }
    
    Ok(())
}