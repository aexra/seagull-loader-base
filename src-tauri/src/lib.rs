// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[tauri::command]
async fn update() -> Result<(), String> {
    // Считать источник клиента из локального файла conf.json
    // Проверить во вложенной папке client наличие файла conf.json
    // Если файла нет или версия не актуальна выполнть обновление
    // Если актуальная версия - закрыть приложение и запустить клиент

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
