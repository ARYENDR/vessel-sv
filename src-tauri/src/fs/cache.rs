use std::fs;

use toml::Table;

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
pub fn set_cache(k: &str, v: &str) {
    let project_dir = directories::ProjectDirs::from("org", "vxil", "vessel")
        .expect("home not found, digitally homeless, i see...");
    let mut cache_toml = get_cache();
    cache_toml.insert(k.to_string(), toml::Value::String(v.to_string()));
    let cache_str = toml::to_string(&cache_toml).unwrap();
    fs::write(project_dir.config_dir().join("vesconfig.toml"), cache_str);
}

fn get_cache() -> Table {
    let project_dir =
        directories::ProjectDirs::from("org", "vxil", "vessel").expect("no home found... weird...");
    let toml_str = fs::read_to_string(project_dir.config_dir().join("vesconfig.toml")).unwrap();
    let cache_toml: Table = toml::from_str(toml_str.as_str()).unwrap();
    cache_toml
}

#[tauri::command]
pub fn get_cache_by_key(k: &str) -> String {
    let toml_cache = get_cache();
    let val = toml_cache.get(k).expect("no value");
    val.to_string()
}
