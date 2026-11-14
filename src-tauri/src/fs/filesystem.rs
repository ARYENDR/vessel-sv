use std::{fs, path::Path};

use lofty::{
    file::{AudioFile, TaggedFileExt},
    read_from_path,
    tag::{Accessor, ItemKey::*},
};

use crate::audiosys::audio::VesAudio;

// decode and demux with symphonia
// play audio with cpal
// read/write meta with lofty

#[tauri::command]
pub fn get_files(path: &str) -> Result<Vec<VesAudio>, String> {
    let audio_extensions = ["mp3", "ogg", "flac", "wav"];
    let path = Path::new(path);

    let mut audio_vec: Vec<VesAudio> = vec![];

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(ent) = entry {
                if ent.path().extension().is_some_and(|x| {
                    // checks if the file is an audio file (by audio_extensions)
                    audio_extensions.contains(&x.to_ascii_lowercase().to_str().unwrap())
                }) {
                    let tagged_file = read_from_path(ent.path()).expect("Fail to read");
                    if let Some(tag) = tagged_file.primary_tag() {
                        let aud = (VesAudio::new(
                            tag.title().unwrap().to_string(),
                            tag.album().unwrap().to_string(),
                            tag.get_items(TrackArtists)
                                .map(|x| x.clone().into_value().into_string().unwrap())
                                .collect(),
                            tag.track().unwrap(),
                            ent.path().to_str().unwrap().to_string(),
                            tag.pictures()
                                .first()
                                .ok_or("Track has no cover")
                                .expect("Something happened")
                                .data()
                                .to_vec(),
                            tagged_file.properties().duration().as_secs_f64(),
                            ent.path().to_str().unwrap().to_string(),
                            String::from("hello"),
                            7,
                        ));

                        audio_vec.push(aud);
                    }
                }
            }
        }
    }
    Ok(audio_vec)
}
