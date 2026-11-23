use std::fs;

// checks if dir exists, create dir. check if file exists, create file
pub fn cache_control() -> Result<(), String> {
    let project_dir = directories::ProjectDirs::from("org", "vxil", "vessel")
        .expect("home not found, how do you manage this");

    if let Ok(_) = fs::create_dir(project_dir.config_dir()) {
        fs::write(project_dir.clone().config_dir().join("vesconfig.toml"), "")
            .expect("fail to create file");
    } else if !fs::exists(project_dir.config_dir().join("vesconfig.toml")).expect("fail to check") {
        fs::write(project_dir.clone().config_dir().join("vesconfig.toml"), "")
            .expect("fail to create file");
    }

    Ok(())
}

#[tauri::command]
pub fn set_cache() {}

#[tauri::command]
pub fn get_cache() {}
