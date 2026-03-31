use tauri_plugin_shell::ShellExt;

// 1. Definimos nuestro comando de descarga/check
#[tauri::command]
async fn check_video_url(app: tauri::AppHandle, url: String) -> Result<String, String> {
    // Invocamos el sidecar configurado en tauri.conf.json
    let sidecar_command = app.shell()
        .sidecar("yt-dlp")
        .map_err(|e| format!("Error al cargar sidecar: {}", e))?
        .args(["--get-title", &url]);

    let output = sidecar_command.output().await
        .map_err(|e| format!("Error de ejecución: {}", e))?;

    if output.status.success() {
        let title = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(title)
    } else {
        let error_msg = String::from_utf8_lossy(&output.stderr).to_string();
        Err(error_msg)
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // Mantenemos opener si quieres abrir carpetas después, 
        // pero añadimos el plugin de shell que es vital para yt-dlp
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init()) 
        .invoke_handler(tauri::generate_handler![
            check_video_url // <--- Registramos AQUÍ nuestro comando
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}