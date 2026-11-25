use tauri::{Manager, http::Response};

use crate::{
    audiosys::playback::spawn_audio_thread,
    vessel_fs::{cache::cache_control, filesystem::get_cover},
};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod audiosys;
mod vessel_fs;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    cache_control().unwrap();
    tauri::Builder::default()
        .setup(|app| {
            let aud_tx = spawn_audio_thread(app.handle());
            app.manage(aud_tx);
            Ok(())
        })
        // .manage(|app| spawn_audio_thread(&app)) // spawns audio thread every boot
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .register_asynchronous_uri_scheme_protocol("covers", |_, request, responder| {
            let id = request.uri().path().trim_start_matches("/").to_string();
            println!("{}", id);

            tauri::async_runtime::spawn_blocking(move || {
                let response = match get_cover(&id) {
                    Some(b) => Response::builder()
                        .header("content-type", "image/jpeg")
                        .header("Cache-Control", "max-age=99990")
                        .body(b)
                        .unwrap(),
                    None => Response::builder().status(404).body(Vec::new()).unwrap(),
                };
                responder.respond(response);
            });
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            vessel_fs::filesystem::get_audio_meta,
            audiosys::playback::playback,
            vessel_fs::cache::get_cache_by_key,
            vessel_fs::cache::set_cache,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
