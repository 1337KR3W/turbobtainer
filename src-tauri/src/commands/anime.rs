use regex::Regex;
use reqwest::header::USER_AGENT;
use scraper::{Html, Selector};
use serde::{Serialize, Deserialize};

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
    println!(">>> RUST RECIBIÓ BÚSQUEDA: {}", query); // Mira la consola de la terminal
    let url = format!("https://www3.animeflv.net/browse?q={}", query.replace(" ", "+"));
    let client = reqwest::Client::new();

    let response = client
        .get(&url)
        .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?
        .text()
        .await
        .map_err(|e| format!("Read error: {}", e))?;

    let document = Html::parse_document(&response);
    
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
                description: None,
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

    let re_episodes = Regex::new(r"var episodes = (\[\[.*\]\]);").unwrap();
    
    // Capturamos el ID de la serie para reconstruir la URL (AnimeFLV lo necesita)
    // Suele estar en: var anime_info = ["ID", "Titulo", "slug"];
    let _re_info = Regex::new(r###"var anime_info = \["(\d+)",\s*"([^"]+)",\s*"([^"]+)"\];"###).unwrap();
    
    let mut episodes_list = Vec::new();

    if let Some(caps) = re_episodes.captures(&response) {
        let json_data = &caps[1];
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
    episodes_list.reverse();

    Ok(episodes_list)
}

#[tauri::command]
pub async fn get_stream_link(url: String) -> Result<StreamSource, String> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .build().map_err(|e| e.to_string())?;

    let res = client.get(&url).send().await.map_err(|e| e.to_string())?.text().await.map_err(|e| e.to_string())?;
    let re_videos = Regex::new(r"var videos = (\{.*\});").unwrap();
    let caps = re_videos.captures(&res).ok_or("No se encontraron servidores")?;
    let v: serde_json::Value = serde_json::from_str(&caps[1]).map_err(|e| e.to_string())?;

    let servers = v["SUB"].as_array().ok_or("No hay lista de servidores")?;
    
    for s in servers {
        let server_id = s["server"].as_str().unwrap_or("");
        let iframe_url = s["code"].as_str().unwrap_or("");

        if server_id == "sw" {
            let video_id = iframe_url.split('/').last().unwrap_or("");
            
            // Nota: Este patrón cambia a veces, pero es el más estable
            let direct_url = format!("https://awentub.com/stream/{}/master.m3u8", video_id);

            // Verificamos si el link responde bien (HEAD request para no bajar todo el archivo)
            let check = client.head(&direct_url)
                .header("Referer", "https://streamwish.to/")
                .send().await;

            if let Ok(response) = check {
                if response.status().is_success() {
                    return Ok(StreamSource {
                        server: "Streamwish_Direct".to_string(),
                        url: direct_url,
                        quality: Some("Auto".to_string()),
                    });
                }
            }
            
            // Si el "adivinar" falla, devolvemos el iframe como último recurso
            // pero esta vez nos aseguramos de que el Frontend lo maneje
            return Ok(StreamSource {
                server: "Streamwish_Iframe".to_string(),
                url: iframe_url.to_string(), // https://streamwish.to/e/vi3iu795r3h6
                quality: None,
            });
        }
    }
    Err("Servidor no compatible".into())
}