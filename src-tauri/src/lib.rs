use tauri_plugin_shell::ShellExt;
use tauri::{Emitter, Manager};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
mod anime;

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
        Err(stderr.trim().to_string())
    }
}
#[tauri::command]
async fn download_video(app: tauri::AppHandle, url: String, tipo: String) -> Result<String, String> {
    
    // --- 1. PREPARAR EL SIDECAR DE FFmpeg ---
    // En lugar de buscar un archivo en una carpeta, le pedimos a Tauri su sidecar oficial.
    let _ffmpeg_sidecar = app.shell().sidecar("ffmpeg")
        .map_err(|e| format!("FFmpeg Sidecar Error: {}", e))?;

    // --- 2. DIRECTORIO DE DESCARGA ---
    let download_dir = app.path().download_dir()
        .map_err(|e| format!("Download path not found: {}", e))?;

    let output_str = download_dir.join("%(title).150s_Turbobtainer_%(epoch)s.%(ext)s")
        .to_string_lossy().to_string();

    // --- 3. CONFIGURAR ARGUMENTOS ---
    let mut args = vec![
        "--newline",
        "--progress",
        "--progress-template", "PROG:%(progress._percent_str)s",
        "--no-playlist",
        "--no-overwrites",
        "-o", &output_str,
        // IMPORTANTE: Al ser FFmpeg un sidecar, simplemente pasamos el nombre
        // que definimos en tauri.conf.json. Tauri se encarga de inyectar la ruta real.
        "--ffmpeg-location", "ffmpeg", 
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

    // --- 4. EJECUCIÓN ---
    let (mut rx, _child) = app.shell()
        .sidecar("yt-dlp")
        .map_err(|e| format!("Sidecar error yt-dlp: {}", e))?
        .args(args)
        .spawn()
        .map_err(|e| format!("Error initializing process...: {}", e))?;

    // ... (Tu lógica de escucha de eventos se mantiene igual)
    tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            // ... resto del bucle while igual ...
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
        .map_err(|e| format!("Error loading sidecar: {}", e))?;

    let output = sidecar
        .args(["--version"])
        .output()
        .await
        .map_err(|e| format!("Execution error: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}


#[tauri::command]
async fn check_gallery_url(app: tauri::AppHandle, url: String) -> Result<GalleryMetadata, String> {
    let sidecar = app.shell().sidecar("gallery-dl")
        .map_err(|e| format!("Engine Error: {}", e))?;

    // Mantenemos el User-Agent para evitar bloqueos básicos
    let output = sidecar
        .args([
            "-j",
            "--opt", "http.user-agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
            &url
        ])
        .output()
        .await
        .map_err(|e| format!("Process Error: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    //println!("DEBUG RAW STDOUT: {}", stdout);
    if !stderr.is_empty() { println!("DEBUG STDERR: {}", stderr); }

    let mut urls_found = Vec::new();
    let mut title = String::from("Online Gallery");

    // Intentamos parsear cada línea de forma independiente
    for line in stdout.lines() {
        let trimmed = line.trim();
        if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(trimmed) {
            extract_recursive(&json_val, &mut urls_found, &mut title);
        }
    }

    // Si después de las líneas sigue vacío, intentamos el bloque entero
    if urls_found.is_empty() {
        if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(&stdout) {
            extract_recursive(&json_val, &mut urls_found, &mut title);
        }
    }

    if !urls_found.is_empty() {
        urls_found.sort();
        urls_found.dedup();
        let count = urls_found.len();
        
        Ok(GalleryMetadata {
            title: if title.is_empty() { "Gallery".into() } else { title },
            thumbnail: "".into(),
            count,
            description: format!("Success: {} items found.", count)
        })
    } else {
        // Si no hay URLs, devolvemos el error de gallery-dl si existe
        let msg = if !stderr.is_empty() { 
            stderr.to_string() 
        } else if stdout.contains("AuthRequired") {
            "Provider requires Login/API Key.".into()
        } else {
            "No media found in the response.".into()
        };
        Err(msg)
    }
}



fn extract_recursive(v: &serde_json::Value, urls: &mut Vec<String>, title: &mut String) {
    match v {
        serde_json::Value::Object(map) => {
            // 1. GESTIÓN DE TÍTULO
            if *title == "Online Gallery" || title.is_empty() {
                // Prioridad de campos de título
                let title_keys = ["title", "grid_title", "name", "seo_title"];
                for k in title_keys {
                    if let Some(t) = map.get(k).and_then(|val| val.as_str()) {
                        if !t.is_empty() {
                            *title = t.split('|').next().unwrap_or(t).trim().to_string();
                            break;
                        }
                    }
                }
            }
            //URL management
            if let Some(s_obj) = map.get("s").and_then(|v| v.as_object()) {
                if let Some(url_orig) = s_obj.get("u").and_then(|v| v.as_str()) {
                    if url_orig.starts_with("http") {
                        urls.push(url_orig.to_string());
                        return; // IMPORTANTE: Si ya encontramos la original, no entramos a "p" (previews)
                    }
                }
            }
            

            // 2. GESTIÓN DE URLs (FILTRADO)
            for (key, val) in map {
                if let Some(u) = val.as_str() {
                    if u.starts_with("http") {
                        // Filtros de calidad que ya tenías
                        let is_image = u.contains('.') || u.contains("m3u8");
                        let is_not_junk = !u.contains("/75x75") && !u.contains("/30x30") && !u.contains("/upload/");
                        
                        // Si es Pinterest, mantenemos tu filtro de /originals/
                        if u.contains("pinimg.com") {
                            if u.contains("/originals/") {
                                urls.push(u.to_string());
                            }
                        } else if is_image && is_not_junk {
                            // Para el resto, solo añadimos si la clave no sugiere "miniatura" o "preview"
                            let key_lower = key.to_lowercase();
                            if !key_lower.contains("thumb") && !key_lower.contains("preview") && key != "p" {
                                urls.push(u.to_string());
                            }
                        }
                    }
                }
            }

            for (key, val) in map {
                // Evitamos entrar en el array "p" de Reddit para no contar miniaturas
                if key != "p" {
                    extract_recursive(val, urls, title);
                }
            }
        }
        serde_json::Value::Array(list) => {
            for val in list {
                extract_recursive(val, urls, title);
            }
        }
        _ => {}
    }
}

#[tauri::command]
async fn download_gallery(app: tauri::AppHandle, url: String, total_items: usize) -> Result<String, String> {
    let download_dir = app.path().download_dir()
        .map_err(|e| format!("Path Error: {}", e))?;

    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let full_path = download_dir.join(format!("Turbobtainer_{}", timestamp));

    fs::create_dir_all(&full_path).map_err(|e| format!("Folder Error: {}", e))?;

    let sidecar = app.shell().sidecar("gallery-dl")
        .map_err(|e| format!("Engine Error: {}", e))?;

    let (mut rx, _child) = sidecar
        .args([
            "--user-agent", "Turbobtainer/1.2.5 (https://github.com/1337KR3W/turbobtainer; contacto@email.com) gallery-dl/1.26.1",
            "--limit-rate", "2M",
            "-d", &full_path.to_string_lossy(), 
            &url
        ])
        .spawn()
        .map_err(|e| format!("Download Execution Error: {}", e))?;

    let mut downloaded_count = 0;

    use tauri_plugin_shell::process::CommandEvent;
    
    while let Some(event) = rx.recv().await {
        if let CommandEvent::Stdout(line_bytes) = event {
            let line = String::from_utf8_lossy(&line_bytes);
            
            // gallery-dl imprime la ruta del archivo cuando termina de descargarlo
            // Buscamos líneas que contengan la ruta de descarga para contar
            if line.contains(&full_path.to_string_lossy().as_ref()) {
                downloaded_count += 1;
                
                // Calculamos el progreso (0.0 a 1.0)
                let progress = (downloaded_count as f64 / total_items as f64).min(0.99);
                let _ = app.emit("download-progress", progress);
            }
        }
    }

    // Al salir del bucle, el proceso ha terminado
    let has_files = fs::read_dir(&full_path).map(|mut d| d.next().is_some()).unwrap_or(false);
    
    if has_files {
        let _ = app.emit("download-progress", 1.0);
        Ok(format!("Saved to: {}", full_path.display()))
    } else {
        Err("No files were downloaded.".into())
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