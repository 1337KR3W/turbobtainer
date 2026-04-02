use tauri_plugin_shell::ShellExt;
use tauri::{Emitter, Manager};

#[tauri::command]
async fn check_video_url(app: tauri::AppHandle, url: String) -> Result<String, String> {
    if url.trim().is_empty() {
        return Err("The URL cannot be empty.".into());
    }

    let sidecar = app.shell().sidecar("yt-dlp").map_err(|e| {
        format!("SISTEM_ERROR: Download engine not available. ({})", e)
    })?;

    let output = sidecar
        .args(["--get-title", "--no-playlist", "--skip-download", &url])
        .output()
        .await
        .map_err(|e| format!("EXECUTION_ERROR: Could not initiate analysis. ({})", e))?;

    if output.status.success() {
        let title = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if title.is_empty() {
            Err("CONTENT_ERROR: Could not extract title.".into())
        } else {
            Ok(title)
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("SISTEM_ERROR: {}", stderr.trim()))
    }
}

#[tauri::command]
async fn download_video(app: tauri::AppHandle, url: String, tipo: String) -> Result<String, String> {
    let download_dir = app.path().download_dir()
        .map_err(|e| format!("Download directory not found: {}", e))?;
    
    // 1. Detectamos el triple del sistema
    let target_triple = tauri::utils::platform::target_triple().unwrap_or_default();
    let ffmpeg_name = if cfg!(windows) {
        format!("ffmpeg-{}.exe", target_triple)
    } else {
        format!("ffmpeg-{}", target_triple)
    };

    // 2. LÓGICA DE RESOLUCIÓN HÍBRIDA
    // Intento A: Ruta de recursos oficial (Producción)
    let mut ffmpeg_exe_path = app.path().resolve(
        format!("bin/{}", ffmpeg_name), 
        tauri::path::BaseDirectory::Resource
    ).unwrap_or_default();

    // Intento B: Ruta local de desarrollo (si el A no existe)
    if !ffmpeg_exe_path.exists() {
        if let Ok(current_dir) = std::env::current_dir() {
            // En desarrollo, 'current_dir' suele ser la raíz de 'src-tauri'
            let local_path = current_dir.join("bin").join(&ffmpeg_name);
            if local_path.exists() {
                ffmpeg_exe_path = local_path;
            }
        }
    }

    let ffmpeg_str = ffmpeg_exe_path.to_string_lossy().to_string();

    // Verificación final antes de lanzar yt-dlp
    if !ffmpeg_exe_path.exists() {
        return Err(format!("FFmpeg CRITICAL ERROR: Binaries not found in path: {}", ffmpeg_str));
    }
    
    println!("✅ FFmpeg activo para Turbobtainer en: {}", ffmpeg_str);

    let output_path = download_dir.join("Turbobtainer_%(title).150s_%(epoch)s.%(ext)s");
    let output_str = output_path.to_string_lossy().to_string();

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
                                        // Enviamos el progreso, pero limitamos a 0.99 
                                        // para que el SUCCESS solo ocurra al final real del proceso
                                        if val < 1.0 {
                                            let _ = app.emit("download-progress", val);
                                        } else {
                                            // Si es 1.0 por descarga, enviamos 0.99 para indicar "procesando/convirtiendo"
                                            let _ = app.emit("download-progress", 0.99);
                                        }
                                    }
                                }
                            }
                        },
                        tauri_plugin_shell::process::CommandEvent::Stderr(line) => {
                            println!("YT-DLP Log: {}", String::from_utf8_lossy(&line));
                        },
                        // El evento Terminated indica que yt-dlp CERRÓ (terminó de convertir y borrar temporales)
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