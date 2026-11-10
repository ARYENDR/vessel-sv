use std::{fs, path::Path};

#[tauri::command]
pub fn get_files(path: &str) {
    let path = Path::new(path);
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(ent) = entry {
                println!("{}", ent.file_name().into_string().unwrap());
            }
        }
    }
}
