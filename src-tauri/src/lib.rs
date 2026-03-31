use tauri_plugin_shell::ShellExt;
use tauri::{Emitter, Manager};
use std::str;

#[tauri::command]
async fn check_video_url(app: tauri::AppHandle, url: String) -> Result<String, String> {
    if url.trim().is_empty() {
        return Err("La URL no puede estar vacía.".into());
    }

    let sidecar = app.shell().sidecar("yt-dlp").map_err(|e| {
        format!("ERROR_SISTEMA: El motor de descarga no está disponible. ({})", e)
    })?;

    let output = sidecar
        .args(["--get-title", "--no-playlist", "--skip-download", &url])
        .output()
        .await
        .map_err(|e| format!("ERROR_EJECUCION: No se pudo iniciar el análisis. ({})", e))?;

    if output.status.success() {
        let title = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if title.is_empty() {
            Err("ERROR_CONTENIDO: No se pudo extraer el título.".into())
        } else {
            Ok(title)
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("ERROR: {}", stderr.trim()))
    }
}

#[tauri::command]
async fn download_video(app: tauri::AppHandle, url: String, tipo: String) -> Result<String, String> {
    // 1. Ruta de Descargas
    let download_dir = app.path().download_dir()
        .map_err(|e| format!("No se encontró la carpeta de descargas: {}", e))?;
    
    let output_path = download_dir.join("TurboTainer_%(title).200s.%(ext)s");
    let output_str = output_path.to_string_lossy();

    // 2. Localizar la carpeta donde están los binarios (sidecars)
    // En Tauri 2, los recursos se encuentran en el resolve_resource
    // Pero la forma más fiable para yt-dlp es pasarle el nombre del sidecar 
    // y dejar que el PATH interno de la shell de Tauri lo encuentre, 
    // o simplemente no pasarle nada si están en la misma carpeta.
    
    // Sin embargo, para ser precisos con el error anterior, vamos a omitir el path 
    // absoluto complejo y usar una estrategia de "Directorio de Ejecución".
    
    let mut args = vec![
        "--newline",
        "--progress",
        "--progress-template", "download:%(progress._percent_str)s",
        "--no-playlist",
        // Eliminamos la detección manual de ruta que falló y usamos una ruta relativa
        // yt-dlp suele buscar ffmpeg en el mismo directorio donde él reside.
        "-o", &output_str,
    ];

    if tipo == "audio" {
        args.extend(["-x", "--audio-format", "mp3"]);
    }
    
    args.push(&url);

    // 3. Lanzar yt-dlp
    let (mut rx, _child) = app.shell()
        .sidecar("yt-dlp")
        .map_err(|e| format!("Error con sidecar yt-dlp: {}", e))?
        .args(args)
        .spawn()
        .map_err(|e| format!("Error al iniciar descarga: {}", e))?;

    // 4. Hilo de escucha (Mismo código de antes)
    tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                tauri_plugin_shell::process::CommandEvent::Stdout(line) => {
                    let out = String::from_utf8_lossy(&line);
                    if out.contains("download:") {
                        if let Some(pct_part) = out.split(':').nth(1) {
                            let clean_pct: String = pct_part.chars()
                                .filter(|c| c.is_digit(10) || *c == '.')
                                .collect();
                            if let Ok(pct_f) = clean_pct.parse::<f32>() {
                                let _ = app.emit("download-progress", pct_f / 100.0);
                            }
                        }
                    }
                },
                tauri_plugin_shell::process::CommandEvent::Stderr(line) => {
                    eprintln!("🔴 YT-DLP LOG: {}", String::from_utf8_lossy(&line));
                },
                _ => {}
            }
        }
        let _ = app.emit("download-progress", 1.0);
    });

    Ok("Proceso iniciado".into())
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