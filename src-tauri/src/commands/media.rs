use std::path::PathBuf;

use tauri_plugin_shell::ShellExt;
use tauri::{AppHandle, Emitter, Manager};

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
    filesize_approx: Option<f64>,
    playlist_title: Option<String>
}

fn get_ffmpeg_path(app: &AppHandle) -> Result<PathBuf, String> {
    let target_triple = tauri::utils::platform::target_triple().unwrap_or_default();
    let ffmpeg_name = format!("ffmpeg-{}.exe", target_triple);

    // --- CASO A: MODO DESARROLLO (debug) ---
    #[cfg(debug_assertions)]
    {
        // En dev, forzamos la ruta absoluta hacia tu carpeta de proyecto
        let dev_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("bin")
            .join(&ffmpeg_name);
        
        if dev_path.exists() {
            return Ok(dev_path);
        }
    }

    // --- CASO B: MODO PRODUCCIÓN (release) ---
    #[cfg(not(debug_assertions))]
    {
        // 1. Intentar al lado del ejecutable (Instalación plana)
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                let prod_path = exe_dir.join(&ffmpeg_name);
                if prod_path.exists() { return Ok(prod_path); }
            }
        }

        // 2. Intentar carpeta de recursos (Estructura estándar de Tauri bundle)
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
pub async fn download_video(app: AppHandle, url: String, stype: String, download_playlist: bool) -> Result<String, String> {
    let ffmpeg_path = get_ffmpeg_path(&app)?;
    let download_dir = app.path().download_dir().map_err(|e| e.to_string())?;
    
    let output_tmpl = if download_playlist {
        download_dir
            .join("Turbobtainer_%(epoch)s")
            .join("%(playlist_title).100s")
            .join("%(title).100s.%(ext)s")
    } else {
        download_dir.join("%(title).150s_Turbobtainer_%(epoch)s.%(ext)s")
    };

    let output_str = output_tmpl.to_string_lossy().to_string();

    let mut args = vec![
        "--newline", "--progress",
        "--progress-template", "PROG:%(progress._percent_str)s",
        "-o", &output_str,
        "--ffmpeg-location", ffmpeg_path.to_str().ok_or("Invalid path")?,
        "--restrict-filenames",
    ];

    // Lógica crucial: Si NO quiere playlist, hay que decírselo explícitamente a yt-dlp
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

    let (mut rx, _) = app.shell().sidecar("yt-dlp")
        .map_err(|e| e.to_string())?
        .args(args)
        .spawn()
        .map_err(|e| e.to_string())?;

    // --- PROCESAMIENTO DE EVENTOS ---
    tauri::async_runtime::spawn(async move {
        let mut last_emit = 0.0; // Para evitar saturación si quisieras filtrar por umbral

        while let Some(event) = rx.recv().await {
            match event {
                tauri_plugin_shell::process::CommandEvent::Stdout(line) => {
                    let out = String::from_utf8_lossy(&line);
                    
                    // Búsqueda eficiente del patrón PROG:
                    if let Some(pos) = out.find("PROG:") {
                        let pct_str = &out[pos + 5..].trim_end_matches('%').trim();
                        if let Ok(pct) = pct_str.parse::<f32>() {
                            let current_prog = (pct / 100.0).min(0.99);
                            // Solo emitimos si hay un cambio real significativo (ej. > 1%)
                            if current_prog > last_emit + 0.01 || current_prog >= 0.99 {
                                let _ = app.emit("download-progress", current_prog);
                                last_emit = current_prog;
                            }
                        }
                    }
                },
                tauri_plugin_shell::process::CommandEvent::Terminated(payload) => {
                    if payload.code == Some(0) {
                        let _ = app.emit("download-progress", 1.0);
                    }
                },
                _ => {}
            }
        }
    });

    Ok("Download started".into())
}