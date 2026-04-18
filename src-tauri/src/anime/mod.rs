use serde::{Deserialize, Serialize};
use scraper::{Html, Selector};
use reqwest::header::USER_AGENT;
use regex::Regex;

// --- Modelos de Datos (Coinciden con anime.model.ts) ---

#[derive(Debug, Serialize, Deserialize)]
pub struct Anime {
    pub title: String,
    pub url: String,
    pub thumbnail: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Episode {
    pub number: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamSource {
    pub server: String,
    pub url: String,
    pub quality: Option<String>,
}

#[tauri::command]
pub async fn search_anime(query: String) -> Result<Vec<Anime>, String> {
    let url = format!("https://www3.animeflv.net/browse?q={}", query.replace(" ", "+"));
    let client = reqwest::Client::new();

    // Hacemos la petición fingiendo ser un navegador moderno
    let response = client
        .get(&url)
        .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?
        .text()
        .await
        .map_err(|e| format!("Read error: {}", e))?;

    // Parseamos el HTML
    let document = Html::parse_document(&response);
    
    // Selectores CSS
    let item_selector = Selector::parse("ul.ListAnimes li article.Anime").unwrap();
    let title_selector = Selector::parse("h3.Title").unwrap();
    let link_selector = Selector::parse("a").unwrap();
    let img_selector = Selector::parse(".Image img").unwrap();

    let mut results = Vec::new();

    for element in document.select(&item_selector) {
        let title = element
            .select(&title_selector)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default();

        let link = element
            .select(&link_selector)
            .next()
            .and_then(|el| el.value().attr("href"))
            .map(|href| format!("https://www3.animeflv.net{}", href))
            .unwrap_or_default();

        let thumbnail = element
            .select(&img_selector)
            .next()
            .and_then(|el| el.value().attr("src"))
            .unwrap_or_default()
            .to_string();

        if !title.is_empty() && !link.is_empty() {
            results.push(Anime {
                title,
                url: link,
                thumbnail,
                description: None, // La descripción suele requerir entrar en la ficha técnica
            });
        }
    }

    Ok(results)
}

#[tauri::command]
pub async fn get_anime_episodes(url: String) -> Result<Vec<Episode>, String> {
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .send()
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())?;

    // Regex para capturar el array de episodios: var episodes = [[1,1],[2,2],...];
    let re_episodes = Regex::new(r"var episodes = (\[\[.*\]\]);").unwrap();
    
    // Capturamos el ID de la serie para reconstruir la URL (AnimeFLV lo necesita)
    // Suele estar en: var anime_info = ["ID", "Titulo", "slug"];
    let re_info = Regex::new(r###"var anime_info = \["(\d+)",\s*"([^"]+)",\s*"([^"]+)"\];"###).unwrap();
    
    let mut episodes_list = Vec::new();

    if let Some(caps) = re_episodes.captures(&response) {
        let json_data = &caps[1];
        // Parseamos el string como un array de arrays: [[numero_ep, id_ep], ...]
        let raw_episodes: Vec<Vec<serde_json::Value>> = serde_json::from_str(json_data)
            .map_err(|e| format!("JSON Error: {}", e))?;

        // Extraemos el slug de la URL original para montar el link del episodio
        // Ejemplo: https://www3.animeflv.net/anime/solo-leveling -> solo-leveling
        let slug = url.split('/').last().unwrap_or_default();

        for ep in raw_episodes {
            if let (Some(num), Some(_id)) = (ep.get(0), ep.get(1)) {
                let ep_num = num.to_string();
                episodes_list.push(Episode {
                    number: ep_num.clone(),
                    url: format!("https://www3.animeflv.net/ver/{}-{}", slug, ep_num),
                });
            }
        }
    }

    // Los episodios en el script vienen del último al primero, 
    // les damos la vuelta para que el 1 sea el primero.
    episodes_list.reverse();

    Ok(episodes_list)
}



#[tauri::command]
pub async fn get_stream_link(url: String) -> Result<StreamSource, String> {
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .send()
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())?;

    // 1. Buscamos el objeto JSON de servidores en el script
    let re_videos = Regex::new(r"var videos = (\{.*\});").unwrap();
    
    if let Some(caps) = re_videos.captures(&response) {
        let json_data = &caps[1];
        let v: serde_json::Value = serde_json::from_str(json_data).map_err(|e| e.to_string())?;

        // 2. AnimeFLV tiene una lista de servidores bajo la clave "SUB" (subtitulado)
        if let Some(sub_servers) = v["SUB"].as_array() {
            for s in sub_servers {
                let server_name = s["server"].as_str().unwrap_or("");
                let code_url = s["code"].as_str().unwrap_or("");

                // Por ahora, busquemos un servidor compatible con streaming directo
                // 'vidoza' suele ser muy amigable para extraer el .mp4
                if server_name == "vidoza" {
                    // El "code" suele ser un link al iframe. 
                    // Necesitaríamos entrar en ese link para sacar el video, 
                    // pero por ahora devolvamos el link del iframe para probar que lo detectamos.
                    return Ok(StreamSource {
                        server: "Vidoza".to_string(),
                        url: code_url.to_string(),
                        quality: Some("720p".to_string()),
                    });
                }
            }
            
            // Si no hay Vidoza, devolvemos el primero que encontremos como fallback
            if let Some(first) = sub_servers.get(0) {
                return Ok(StreamSource {
                    server: first["server"].as_str().unwrap_or("Unknown").to_string(),
                    url: first["code"].as_str().unwrap_or("").to_string(),
                    quality: None,
                });
            }
        }
    }

    Err("No se encontraron servidores de video disponibles".into())
}