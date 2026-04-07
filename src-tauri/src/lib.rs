use tauri_plugin_shell::ShellExt;
use tauri::{Emitter, Manager};

#[derive(serde::Serialize)]
struct VideoMetadata {
    title: String,
    thumbnail: String,
    duration: String,
    size: String
}

#[tauri::command]
async fn check_video_url(app: tauri::AppHandle, url: String) -> Result<VideoMetadata, String> {
    if url.trim().is_empty() {
        return Err("The URL cannot be empty.".into());
    }

    let sidecar = app.shell().sidecar("yt-dlp").map_err(|e| {
        format!("SYSTEM_ERROR: Download engine not available. ({})", e)
    })?;

    let output = sidecar
        .args([
            "--quiet",
            "--no-warnings", 
            "--no-playlist", 
            "--skip-download",
            "--print", "{\"title\":%(title)j, \"thumbnail\":%(thumbnail)j, \"duration\":%(duration_string)j, \"size\":%(filesize,filesize_approx)j}", 
            &url
            ])

        .output()
        .await
        .map_err(|e| format!("EXECUTION_ERROR: Could not initiate analysis. ({})", e))?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let json_line = stdout.lines().next().unwrap_or("{}");
        let v: serde_json::Value = serde_json::from_str(json_line).map_err(|e| {
            format!("JSON_PARSE_ERROR: {}. Raw: {}", e, json_line)
        })?;
        
        Ok(VideoMetadata {
            // .as_str() maneja las comillas automáticamente
            title: v["title"].as_str().unwrap_or("Unknown Title").to_string(),
            thumbnail: v["thumbnail"].as_str().unwrap_or("").to_string(),
            duration: v["duration"].as_str().unwrap_or("00:00").to_string(),
            // El tamaño puede ser número o string, lo pasamos a string para el frontend
            size: match v["size"].as_f64() {
                Some(bytes) => {
                    let mb = bytes / 1024.0 / 1024.0;
                    format!("{:.2} MB", mb) // Formatea a 2 decimales + el texto " MB"
                },
                None => "Unknown size".to_string(),
            },
        })
        
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("SYSTEM_ERROR: {}", stderr.trim()))
    }
}

#[tauri::command]
async fn download_video(app: tauri::AppHandle, url: String, tipo: String) -> Result<String, String> {
    //RESOLUCIÓN DE RUTA DE FFMPEG
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

    //DIRECTORIO DE DESCARGA Y PARÁMETROS
    let download_dir = app.path().download_dir()
        .map_err(|e| format!("Download directory not found: {}", e))?;
    
    let output_str = download_dir.join("%(title).150s_Turbobtainer_%(epoch)s.%(ext)s")
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

    //EJECUCIÓN
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