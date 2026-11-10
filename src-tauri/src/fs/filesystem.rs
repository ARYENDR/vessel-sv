use super::super::audiosys::audio::VesAudio;
use audiotags::Tag;
use std::{ffi::OsStr, fs, path::Path};

#[tauri::command]
pub fn get_files(path: &str) -> Result<Vec<VesAudio>, String> {
    let path = Path::new(path);
    let audio_list: Vec<VesAudio> = vec![];
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(ent) = entry {
                if ent
                    .path()
                    .extension()
                    .is_some_and(|x| x == OsStr::new("mp3"))
                {
                    println!(
                        "{}",
                        Tag::new()
                            .read_from_path(ent.path())
                            .unwrap()
                            .title()
                            .unwrap()
                    );
                }
            }
        }
    }
    if audio_list.is_empty() {
        Err(String::from("List is empty"))
    } else {
        Ok(audio_list)
    }
}
