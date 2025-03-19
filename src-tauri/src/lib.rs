mod commands;
mod db;
mod crypto;
mod discovery;
mod signaling;
mod file_server;
mod file_transfer;
mod screen_capture;
mod tray;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
