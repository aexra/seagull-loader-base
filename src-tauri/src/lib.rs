// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use serde::{Deserialize, Serialize};
use std::{fs::{self, File}, io::{Read, Write}, path::{Path, PathBuf}};
use reqwest::{self, header::ACCEPT};
use zip::ZipArchive;

#[derive(Serialize, Deserialize)]
struct Config {
    client_source: String
}

#[derive(Serialize, Deserialize)]
struct ClientConfig {
    version: String,
    allow_pre_releases: bool
}

fn load_config(base: &PathBuf) -> Result<Config, Box<dyn std::error::Error>> {
    let path = base
        .join("resources")
        .join("conf.json");
    
    if !path.exists() {
        panic!();
    }

    let json = fs::read_to_string(path)?;
    let data = serde_json::from_str(&json)?;

    Ok(data)
}

fn load_client_config(base: &PathBuf) -> Result<ClientConfig, Box<dyn std::error::Error>> {
    let path = base
        .join("client")
        .join("resources")
        .join("conf.json");

    if !path.exists() {
        panic!();
    }

    let json = fs::read_to_string(path)?;
    let data = serde_json::from_str(&json)?;

    Ok(data)
}

#[tauri::command]
async fn update() -> Result<(), String> {
    // Считать источник клиента из локального файла conf.json
    // Проверить во вложенной папке client наличие файла conf.json
    // Если файла нет или версия не актуальна выполнть обновление
    // Если актуальная версия - закрыть приложение и запустить клиент

    // Для production
    #[cfg(not(debug_assertions))]
    let base_dir = PathBuf::from(".");
    
    // Для development
    #[cfg(debug_assertions)]
    let base_dir = std::env::current_dir().map_err(|e| e.to_string())?.join("target").join("debug");

    let data = load_config(&base_dir).map_err(|e| e.to_string())?;
    let src = data.client_source;

    let client = reqwest::Client::builder()
        .user_agent("Seagull")
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    println!("HttpClient built");

    let response = client.get(&src)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch update: {}", e))?;

    println!("Releases fetched");

    let json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    println!("JSON Deserialized");

    let rel = json
        .as_array()
        .unwrap()
        .first()
        .unwrap();

    println!("Latest version found");

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
        panic!("Error getting version tag");
    };

    let ver = latest_version.strip_prefix('v').unwrap_or(&latest_version);
    println!("Last version: {}", ver);

    // Проверить наличие установленной версии

    println!("Verifing current client version...");

    let client_conf_path = base_dir.join("client").join("resources").join("conf.json");
    if client_conf_path.exists() {
        println!("Config file found");

        // Проверка на версию
        let data = load_client_config(&base_dir);
        
        // Если ошибка - обновимся
        // Если если нет ошибки
        //   и версии совпадают - выйдем
        //   и версия старее - обновимся

        if let Ok(json) = data {
            println!("Config file parsed");
            println!("Comparing versions: {} -> {}", ver, json.version);
            if json.version == ver {
                println!("Version is up to date. Launching client...");
                launch_client();
                return Ok(());
            } else {
                println!("Need upgrade");
            }
        } else {
            let er = data.err();
            println!("Cannot parse json: {}", er.unwrap());
        }
    }

    println!("Updating client...");

    // Установить актуальную версию из release.zip

    // Находим нужный asset
    let assets = rel["assets"]
        .as_array()
        .ok_or("No assets found in release")?;
    println!("Retrieved version assets");

    let release_asset = assets
        .iter()
        .find(|a| a["name"].as_str() == Some("release.zip"))
        .ok_or("release.zip asset not found")?;
    
    let download_url = release_asset["browser_download_url"]
        .as_str()
        .ok_or("Invalid download URL")?;
    println!("release.zip asset found at: {}", download_url);

    // Создаем временную директорию
    let temp_dir = base_dir.join("temp");
    if !temp_dir.exists() {
        fs::create_dir(&temp_dir).map_err(|e| format!("Failed to create temp dir: {}", e))?;
    }
    println!("Temp folder created at: {}", temp_dir.display());

    // Загружаем архив
    let zip_path = temp_dir.join("release.zip");
    println!("Downloading archive to: {}", zip_path.display());

    download_file(download_url, &zip_path, &client).await?;
    println!("Archive downloaded");
    
    println!("Unzipping archive...");

    // Распаковываем архив
    extract_zip(&zip_path, &temp_dir)?;
    println!("Archive unzipped");

    // Удаляем архив
    fs::remove_file(&zip_path).map_err(|e| format!("Failed to remove zip file: {}", e))?;
    println!("Temp archive removed");

    // Удаляем старый клиент, если он вообще был
    fs::remove_dir_all(&base_dir.join("client")).ok();
    println!("Client placeholder cleared");

    // Переименовываем temp в client
    fs::rename(&base_dir.join("temp"), &base_dir.join("client")).ok();
    println!("Temp folder renamed to client");

    launch_client();

    Ok(())
}

async fn download_file(url: &str, path: &Path, client: &reqwest::Client) -> Result<(), String> {
    let mut response = client.get(url)
        .header(ACCEPT, "application/octet-stream")
        .send()
        .await
        .map_err(|e| format!("Failed to download file: {}", e))?;

    let mut file = File::create(path)
        .map_err(|e| format!("Failed to create file: {}", e))?;

    while let Some(chunk) = response.chunk().await.map_err(|e| format!("Failed to read chunk: {}", e))? {
        file.write_all(&chunk)
            .map_err(|e| format!("Failed to write chunk: {}", e))?;
    }

    Ok(())
}

fn extract_zip(zip_path: &Path, target_dir: &Path) -> Result<(), String> {
    let file = File::open(zip_path)
        .map_err(|e| format!("Failed to open zip file: {}", e))?;

    let mut archive = ZipArchive::new(file)
        .map_err(|e| format!("Failed to read zip archive: {}", e))?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)
            .map_err(|e| format!("Failed to get file #{}: {}", i, e))?;

        let outpath = target_dir.join(file.mangled_name());

        if file.name().ends_with('/') {
            // Это директория
            fs::create_dir_all(&outpath)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        } else {
            // Это файл
            if let Some(parent) = outpath.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent)
                        .map_err(|e| format!("Failed to create parent directory: {}", e))?;
                }
            }

            let mut outfile = File::create(&outpath)
                .map_err(|e| format!("Failed to create file: {}", e))?;

            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)
                .map_err(|e| format!("Failed to read zip entry: {}", e))?;

            outfile.write_all(&buffer)
                .map_err(|e| format!("Failed to write file: {}", e))?;
        }
    }

    Ok(())
}

fn launch_client() {
    println!("Should start client")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![update])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
