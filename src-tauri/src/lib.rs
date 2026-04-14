use tauri_plugin_shell::ShellExt;
use tauri::{Emitter, Manager};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(serde::Serialize)]
struct VideoMetadata {
    title: String,
    thumbnail: String,
    duration: String,
    size: String
}

#[derive(serde::Serialize)]
struct GalleryMetadata {
    title: String,
    thumbnail: String,
    count: usize,
    description: String
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
        Err(format!("SYSTEM_ERROR: {}", stderr.trim()))
    }
}

#[tauri::command]
async fn download_video(app: tauri::AppHandle, url: String, tipo: String) -> Result<String, String> {
    
    let target_triple = tauri::utils::platform::target_triple().unwrap_or_default();
    let ffmpeg_file = format!("bin/ffmpeg-{}.exe", target_triple);

    // El PathResolver es la única autoridad fiable para encontrar archivos empaquetados.
    // BaseDirectory::Resource buscará en 'src-tauri' en dev y en la carpeta de instalación en prod.
    let ffmpeg_path = app.path()
        .resolve(&ffmpeg_file, tauri::path::BaseDirectory::Resource)
        .map_err(|e| format!("Infraestructure Error: Conversion engine not found ({})", e))?;

    // 3. Verificación de integridad
    if !ffmpeg_path.exists() {
        return Err(format!(
            "System error: FFmpeg not found in: {}", 
            ffmpeg_path.display()
        ).into());
    }

    let ffmpeg_str = ffmpeg_path.to_string_lossy().to_string();

    // --- DIRECTORIO DE DESCARGA ---
    let download_dir = app.path().download_dir()
    .map_err(|e| format!("Download path not fount: {}", e))?;

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

    // Ejecutamos yt-dlp como sidecar nativo
    let (mut rx, _child) = app.shell()
        .sidecar("yt-dlp")
        .map_err(|e| format!("Sidecar error yt-dlp: {}", e))?
        .args(args)
        .spawn()
        .map_err(|e| format!("Error initializing process...: {}", e))?;

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

#[tauri::command]
async fn check_gallery_binary(app: tauri::AppHandle) -> Result<String, String> {
    let sidecar = app.shell().sidecar("gallery-dl")
        .map_err(|e| format!("Error al cargar sidecar: {}", e))?;

    let output = sidecar
        .args(["--version"])
        .output()
        .await
        .map_err(|e| format!("Error de ejecución: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[tauri::command]
async fn check_gallery_url(app: tauri::AppHandle, url: String) -> Result<GalleryMetadata, String> {
    let sidecar = app.shell().sidecar("gallery-dl")
        .map_err(|e| format!("SYSTEM_ERROR: Engine not available. ({})", e))?;

    // Usamos -j para obtener JSON y --get-urls para listar lo que encontraría
    let output = sidecar
        .args(["-j", "--get-urls", &url])
        .output()
        .await
        .map_err(|e| format!("EXECUTION_ERROR: Failed to analyze gallery. ({})", e))?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // gallery-dl a veces devuelve varias líneas de JSON o logs. 
        // Vamos a contar las líneas que parecen URLs para saber cuántas imágenes hay.
        let lines: Vec<&str> = stdout.lines().collect();
        let image_count = lines.iter().filter(|l| l.starts_with("http")).count();

        // Por ahora, devolvemos una metadata simplificada
        Ok(GalleryMetadata {
            title: "Gallery Content".to_string(), // gallery-dl no siempre da el título fácil en el JSON
            thumbnail: "".to_string(),             // Veremos cómo extraer la primera imagen luego
            count: image_count,
            description: format!("Found {} images available for download.", image_count)
        })
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("GALLERY_ERROR: {}", stderr.trim()))
    }
}

#[tauri::command]
async fn download_gallery(app: tauri::AppHandle, url: String) -> Result<String, String> {
    // 1. Obtener la ruta de Descargas del sistema
    let download_dir = app.path().download_dir()
        .map_err(|e| format!("Could not find download directory: {}", e))?;

    // 2. Generar un nombre único basado en el tiempo
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    
    let folder_name = format!("TurboTainer_Gallery_{}", timestamp);
    let full_path = download_dir.join(folder_name);

    // 3. Crear la carpeta físicamente
    fs::create_dir_all(&full_path)
        .map_err(|e| format!("Failed to create gallery folder: {}", e))?;

    // 4. Configurar el Sidecar de gallery-dl
    let sidecar = app.shell().sidecar("gallery-dl")
        .map_err(|e| format!("Engine not available: {}", e))?;

    // Argumento -d especifica dónde guardar
    // Ejecutamos de forma asíncrona pero esperamos el resultado final (por ahora)
    let output = sidecar
        .args(["-d", &full_path.to_string_lossy(), &url])
        .output()
        .await
        .map_err(|e| format!("Download execution error: {}", e))?;

    if output.status.success() {
        // Al terminar, emitimos el progreso 1.0 para que Angular sepa que terminó
        let _ = app.emit("download-progress", 1.0);
        Ok(format!("Gallery successfully saved to: {}", full_path.display()))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Gallery-dl error: {}", stderr.trim()))
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            check_video_url,
            download_video,
            check_gallery_binary,
            check_gallery_url,
            download_gallery
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}