use crate::audiosys::playback::spawn_audio_thread;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod audiosys;
mod fs;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(spawn_audio_thread())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            fs::filesystem::get_files,
            audiosys::playback::playback,
            fs::cache::get_cache,
            fs::cache::set_cache,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
