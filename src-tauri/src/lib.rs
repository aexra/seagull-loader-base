// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use reqwest::{self};

#[derive(Serialize, Deserialize)]
struct Config {
    client_source: String
}

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let file_path = PathBuf::from("resources/conf.json");

    if !file_path.exists() {
        panic!();
    }

    let json = fs::read_to_string(file_path)?;
    let data = serde_json::from_str(&json)?;

    Ok(data)
}

#[tauri::command]
async fn update() -> Result<(), String> {
    // Считать источник клиента из локального файла conf.json
    // Проверить во вложенной папке client наличие файла conf.json
    // Если файла нет или версия не актуальна выполнть обновление
    // Если актуальная версия - закрыть приложение и запустить клиент

    let data = load_config().map_err(|e| e.to_string())?;
    let src = data.client_source;

    let client = reqwest::Client::builder()
        .user_agent("Seagull/0.0.1-alpha.1")
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let response = client.get(&src)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch update: {}", e))?;

    let json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let latest_version = if let Some(array) = json.as_array() {
        if let Some(first) = array.first() {
            first["tag_name"]
                .as_str()
                .ok_or("tag_name is not a string or missing")?
                .to_string()
        } else {
            return Err("Empty response array".to_string());
        }
    } else {
        json["tag_name"]
            .as_str()
            .ok_or("tag_name is not a string or missing")?
            .to_string()
    };

    println!("Latest version: {}", latest_version);

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![update])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
