use tauri_plugin_shell::ShellExt;
use tauri::Emitter;
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
    // 1. Argumentos mejorados para evitar buffering y forzar formato limpio
    let mut args = vec![
        "--newline",
        "--no-buffer", // Evita que los mensajes se queden atascados en el buffer
        "--progress",
        "--progress-template", "download:%(progress._percent_str)s",
    ];

    if tipo == "audio" {
        args.extend(["-x", "--audio-format", "mp3"]);
    }
    
    args.push(&url);

    let (mut rx, _child) = app.shell()
        .sidecar("yt-dlp")
        .map_err(|e| e.to_string())?
        .args(args)
        .spawn()
        .map_err(|e| e.to_string())?;

    tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            if let tauri_plugin_shell::process::CommandEvent::Stdout(line) = event {
                let out = String::from_utf8_lossy(&line);
                
                if out.contains("download:") {
                    // 2. Parseo ultra-robusto: extraemos solo dígitos y puntos
                    if let Some(pct_part) = out.split(':').nth(1) {
                        let clean_pct: String = pct_part
                            .chars()
                            .filter(|c| c.is_digit(10) || *c == '.')
                            .collect();

                        if let Ok(pct_f) = clean_pct.parse::<f32>() {
                            // Emitimos el valor (0.0 a 1.0)
                            let _ = app.emit("download-progress", pct_f / 100.0);
                        }
                    }
                }
            }
        }
        // Opcional: Al terminar el bucle, emitir 1.0 para asegurar que la barra llegue al final
        let _ = app.emit("download-progress", 1.0);
    });

    Ok("Descarga iniciada".into())
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