mod commands;



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // Comandos de Media (yt-dlp)
            commands::check_video_url,
            commands::download_video,

            // Comandos de Gallery (gallery-dl)
            commands::check_gallery_binary,
            commands::check_gallery_url,
            commands::download_gallery,

            // Comandos de Anime (Scraping)
            commands::anime::search_anime,
            commands::anime::get_anime_episodes,
            commands::anime::get_stream_link
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}