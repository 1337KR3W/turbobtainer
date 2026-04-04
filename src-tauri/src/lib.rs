use tauri_plugin_shell::ShellExt;
use tauri::{Emitter, Manager};


// Añade esta estructura al principio de lib.rs
#[derive(serde::Serialize)]
struct VideoMetadata {
    title: String,
    thumbnail: String,
    duration: String
}

#[tauri::command]
async fn check_video_url(app: tauri::AppHandle, url: String) -> Result<VideoMetadata, String> {
    if url.trim().is_empty() {
        return Err("The URL cannot be empty.".into());
    }

    let sidecar = app.shell().sidecar("yt-dlp").map_err(|e| {
        format!("SYSTEM_ERROR: Download engine not available. ({})", e)
    })?;

    // Pedimos título y miniatura (separados por un salto de línea en la salida)
    let output = sidecar
        .args(["--get-title", 
        "--get-thumbnail",
        "--get-duration", 
        "--no-playlist", 
        "--skip-download", 
        &url])
        .output()
        .await
        .map_err(|e| format!("EXECUTION_ERROR: Could not initiate analysis. ({})", e))?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
        println!("Lineas capturadas: {:?}", lines);
        
        if lines.len() >= 3 {
            Ok(VideoMetadata {
                title: lines[0].trim().to_string(),
                thumbnail: lines[1].trim().to_string(),
                duration: lines[2].trim().to_string(),
            })
        } else if lines.len() == 1 {
             // Caso borde por si falla la miniatura pero tenemos título
             Ok(VideoMetadata {
                title: lines[0].trim().to_string(),
                thumbnail: "".into(),
                duration: "".into(),
            })
        } else {
            Err("CONTENT_ERROR: Could not extract metadata.".into())
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("SYSTEM_ERROR: {}", stderr.trim()))
    }
}

#[tauri::command]
async fn download_video(app: tauri::AppHandle, url: String, tipo: String) -> Result<String, String> {
    // 1. RESOLUCIÓN DE RUTA DE FFMPEG
    let target_triple = tauri::utils::platform::target_triple().unwrap_or_default();
    let ffmpeg_name = format!("ffmpeg-{}.exe", target_triple);
    
    // En desarrollo: buscar en CARGO_MANIFEST_DIR/bin (src-tauri/bin)
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let dev_path = manifest_dir.join("bin").join(&ffmpeg_name);
    
    // En producción: buscar junto al ejecutable
    let exe_path = std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(|p| p.to_path_buf()))
        .map(|dir| dir.join(&ffmpeg_name));
    
    let ffmpeg_path = if dev_path.exists() {
        dev_path
    } else if let Some(exe_p) = exe_path {
        if exe_p.exists() {
            exe_p
        } else {
            return Err("FFmpeg not found. Please reinstall the application.".into());
        }
    } else {
        return Err("FFmpeg not found. Please reinstall the application.".into());
    };
    
    let ffmpeg_str = ffmpeg_path.to_string_lossy().to_string();

    // 2. DIRECTORIO DE DESCARGA Y PARÁMETROS
    let download_dir = app.path().download_dir()
        .map_err(|e| format!("Download directory not found: {}", e))?;
    
    let output_str = download_dir.join("Turbobtainer_%(title).150s_%(epoch)s.%(ext)s")
        .to_string_lossy().to_string();

    let mut args = vec![
        "--newline",
        "--progress",
        "--progress-template", "PROG:%(progress._percent_str)s",
        "--no-playlist",
        "--no-overwrites",
        "-o", &output_str,
        "--ffmpeg-location", &ffmpeg_str,
    ];

    if tipo == "audio" {
        args.extend(["-x", "--audio-format", "mp3", "--audio-quality", "0"]);
    } else {
        args.extend([
            "--format", "bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]/best",
            "--merge-output-format", "mp4"
        ]);
    }

    args.push(&url);

    // 3. EJECUCIÓN
    let (mut rx, _child) = app.shell()
        .sidecar("yt-dlp")
        .map_err(|e| format!("Sidecar error: {}", e))?
        .args(args)
        .spawn()
        .map_err(|e| format!("Process error: {}", e))?;

    tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                tauri_plugin_shell::process::CommandEvent::Stdout(line) => {
                    let out = String::from_utf8_lossy(&line);
                    if out.contains("PROG:") {
                        if let Some(parts) = out.split("PROG:").nth(1) {
                            let clean_num: String = parts.chars()
                                .filter(|c| c.is_ascii_digit() || *c == '.')
                                .collect();
                            if let Ok(pct_f) = clean_num.parse::<f32>() {
                                let val = pct_f / 100.0;
                                let _ = app.emit("download-progress", if val < 1.0 { val } else { 0.99 });
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

    Ok("Download process initiated".into())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            check_video_url,
            download_video
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}