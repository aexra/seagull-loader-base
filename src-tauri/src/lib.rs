// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use serde::{Deserialize, Serialize};
use std::{f32::consts::E, fs, path::PathBuf};

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

    let data = load_config();

    if let Err(_) = data {
        panic!()
    } else {
        let conf = data.unwrap();
        let src = conf.client_source;
    }

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
