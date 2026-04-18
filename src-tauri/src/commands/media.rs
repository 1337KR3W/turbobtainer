use tauri_plugin_shell::ShellExt;
use tauri::{Emitter, Manager};

#[derive(serde::Serialize)]
pub struct VideoMetadata {
    pub title: String,
    pub thumbnail: String,
    pub duration: String,
    pub size: String
}

#[tauri::command]
pub async fn check_video_url(app: tauri::AppHandle, url: String) -> Result<VideoMetadata, String> {
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
            title: v["title"].as_str().unwrap_or("Unknown Title").to_string(),
            thumbnail: v["thumbnail"].as_str().unwrap_or("").to_string(),
            duration: v["duration"].as_str().unwrap_or("00:00").to_string(),
            size: match v["size"].as_f64() {
                Some(bytes) => {
                    let mb = bytes / 1024.0 / 1024.0;
                    format!("{:.2} MB", mb)
                },
                None => "Unknown size".to_string(),
            },
        })
        
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(stderr.trim().to_string())
    }
}

#[tauri::command]
pub async fn download_video(app: tauri::AppHandle, url: String, tipo: String) -> Result<String, String> {
    
    let target_triple = tauri::utils::platform::target_triple().unwrap_or_default();
    let ffmpeg_name = format!("ffmpeg-{}.exe", target_triple);
    
    // --- LÓGICA DE DETECCIÓN DE ENTORNO ---
    
    // 1. Intentamos la ruta de DESARROLLO (src-tauri/bin)
    // Usamos CARGO_MANIFEST_DIR que solo existe cuando compilas/ejecutas con cargo
    let mut ffmpeg_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("bin")
        .join(&ffmpeg_name);

    // 2. Si no existe (estamos en el .msi instalado), buscamos en PRODUCCIÓN
    if !ffmpeg_path.exists() {
        // En producción, el Sidecar suele estar en el mismo directorio que el .exe
        // o en la carpeta de recursos que nos da Tauri.
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                // Opción A: Al lado del ejecutable (común en instalaciones planas)
                let prod_path = exe_dir.join(&ffmpeg_name);
                
                // Opción B: En la carpeta de recursos de Tauri
                let resource_path = app.path().resource_dir()
                    .unwrap_or_default()
                    .join("_up_")
                    .join("bin")
                    .join(&ffmpeg_name);

                if prod_path.exists() {
                    ffmpeg_path = prod_path;
                } else if resource_path.exists() {
                    ffmpeg_path = resource_path;
                }
            }
        }
    }

    // --- VERIFICACIÓN FINAL ---
    if !ffmpeg_path.exists() {
        return Err(format!(
            "FFmpeg no encontrado. Buscado en Dev y Prod. Archivo esperado: {}", 
            ffmpeg_name
        ));
    }

    let ffmpeg_str = ffmpeg_path.to_string_lossy().to_string();

    // --- CONFIGURACIÓN DE DESCARGA ---
    let download_dir = app.path().download_dir()
        .map_err(|e| format!("Error en ruta de descargas: {}", e))?;

    let output_str = download_dir.join("%(title).150s_Turbobtainer_%(epoch)s.%(ext)s")
        .to_string_lossy().to_string();

    let mut args = vec![
        "--newline", "--progress",
        "--progress-template", "PROG:%(progress._percent_str)s",
        "--no-playlist", "--no-overwrites",
        "-o", &output_str,
        "--ffmpeg-location", &ffmpeg_str, // LA RUTA QUE HEMOS CALCULADO
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

    // --- EJECUCIÓN ---
    let (mut rx, _child) = app.shell()
        .sidecar("yt-dlp")
        .map_err(|e| format!("Sidecar error yt-dlp: {}", e))?
        .args(args)
        .spawn()
        .map_err(|e| format!("Error al iniciar proceso: {}", e))?;

    // (El resto del tauri::async_runtime::spawn se queda igual...)
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

    Ok("Descarga iniciada".into())
}