use tauri_plugin_shell::ShellExt;
use tauri::Manager; // Necesario para algunas funciones de AppHandle

#[tauri::command]
async fn check_video_url(app: tauri::AppHandle, url: String) -> Result<String, String> {
    // Validación de URL
    if !url.contains("youtube.com") && !url.contains("youtu.be") {
        return Err("La URL no es de YouTube.".into());
    }

    // Configuración del Sidecar
    let sidecar_command = app.shell()
        .sidecar("yt-dlp")
        .map_err(|e| format!("Error Sidecar: {}", e))?
        .args([
            "--get-title", 
            "--no-playlist", 
            "--skip-download", 
            &url
        ]);

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
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
                .invoke_handler(tauri::generate_handler![
            check_video_url
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}