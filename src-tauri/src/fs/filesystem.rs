use audiotags::Tag;
use std::{fs, path::Path};

use crate::audiosys::audio::VesAudio;

#[tauri::command]
pub fn get_files(path: &str) -> Result<Vec<VesAudio>, String> {
    let audio_extensions = ["mp3", "ogg", "flac", "wav"];
    let path = Path::new(path);
    let mut audio_list: Vec<VesAudio> = vec![];
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(ent) = entry {
                if ent.path().extension().is_some_and(|x| {
                    // checks if the file is an audio file (by audio_extensions)
                    audio_extensions.contains(&x.to_ascii_lowercase().to_str().unwrap())
                }) {
                    println!("{}", ent.path().display());
                    let tagged = Tag::new().read_from_path(ent.path()).unwrap();
                    let anytagged = &tagged.to_anytag(); // idk why this is necessary, rust gave me an error without this

                    // println!(
                    //     "{} from {} number {} by {} length {} desc {}",
                    //     anytagged.title().unwrap(),
                    //     anytagged.album_title().unwrap(),
                    //     anytagged.track_number().unwrap_or(1),
                    //     anytagged.artists().unwrap().join(" ,"),
                    //     // ent.path().into_string().unwrap(),
                    //     anytagged.duration().unwrap_or(0.0),
                    //     "Nothing lmaoo".to_string()
                    // );

                    audio_list.push(VesAudio::new(
                        anytagged.title().unwrap().to_string(),
                        anytagged.album_title().unwrap().to_string(),
                        anytagged
                            .artists()
                            .unwrap()
                            .to_vec()
                            .to_owned()
                            .into_iter()
                            .map(|x| x.to_owned())
                            .collect(), // all will be okay
                        anytagged.track_number().unwrap_or(1),
                        ent.path().into_string().unwrap(),
                        "cover test".to_string(), // convert this from picture to string, somehow...
                        anytagged.duration().unwrap_or(0.0),
                        ent.path().into_string().unwrap(),
                        "description".to_string(),
                        7,
                    ));
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
