use tauri_plugin_shell::ShellExt;

#[tauri::command]
async fn check_video_url(app: tauri::AppHandle, url: String) -> Result<String, String> {
    // 1. Error de Validación (Lógica de Rust)
    if url.trim().is_empty() {
        return Err("La URL no puede estar vacía.".into());
    }

    // 2. Intento de ejecución del Sidecar
    let sidecar = app.shell().sidecar("yt-dlp").map_err(|e| {
        format!("ERROR_SISTEMA: El motor de descarga no está disponible. ({})", e)
    })?;

    // 3. Captura de salida con manejo de timeout o fallos de proceso
    let output = sidecar
        .args(["--get-title", "--no-playlist", "--skip-download", &url])
        .output()
        .await
        .map_err(|e| format!("ERROR_EJECUCION: No se pudo iniciar el análisis. ({})", e))?;

    // 4. Análisis del código de salida (Exit Status)
    if output.status.success() {
        let title = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if title.is_empty() {
            Err("ERROR_CONTENIDO: No se pudo extraer el título. ¿Es un video privado?".into())
        } else {
            Ok(title)
        }
    } else {
        // 5. Mapeo de errores específicos de yt-dlp
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        if stderr.contains("Incomplete YouTube ID") {
            Err("ERROR_USUARIO: La ID del video es incorrecta.".into())
        } else if stderr.contains("Unable to download webpage") {
            Err("ERROR_RED: Sin conexión a internet o YouTube bloqueó la petición.".into())
        } else {
            Err(format!("ERROR_DESCONOCIDO: {}", stderr.trim()))
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
                .invoke_handler(tauri::generate_handler![
            check_video_url
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}