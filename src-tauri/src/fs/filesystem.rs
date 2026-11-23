use std::{fs, path::Path};

use lofty::{
    file::{AudioFile, TaggedFileExt},
    picture::PictureType::CoverFront,
    read_from_path,
    tag::{Accessor, ItemKey::*},
};

use crate::audiosys::audio::VesAudio;

// just use rodio
// read/write meta with lofty

#[tauri::command]
pub fn get_audio_meta(path: String) -> Vec<VesAudio> {
    let path = path.replace("\"", "");
    let audio_extensions = ["mp3", "ogg", "flac", "wav"];
    let path = Path::new(path.as_str());

    let mut audio_vec: Vec<VesAudio> = vec![];

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(ent) = entry {
                if ent.path().extension().is_some_and(|x| {
                    // checks if the file is an audio file (by audio_extensions)
                    audio_extensions.contains(&x.to_ascii_lowercase().to_str().unwrap())
                }) {
                    let tagged_file = read_from_path(ent.path()).expect("Fail to read");
                    println!("{}", ent.path().display());
                    if let Some(tag) = tagged_file.primary_tag() {
                        let aud = VesAudio::new(
                            tag.title().unwrap().to_string(),
                            tag.album().unwrap().to_string(),
                            tag.get_items(TrackArtists)
                                .map(|x| x.clone().into_value().into_string().unwrap())
                                .collect(),
                            tag.track().unwrap_or(1),
                            ent.path().to_str().unwrap().to_string(),
                            tag.pictures()
                                .iter()
                                .find(|p| p.pic_type() == CoverFront)
                                .or_else(|| tag.pictures().first())
                                .is_some(),
                            tagged_file.properties().duration().as_secs_f64(),
                            ent.path().to_str().unwrap().to_string(),
                            String::from("hello"),
                            7,
                        );

                        audio_vec.push(aud);
                    }
                }
            }
        }
    }
    println!("Finished! sent!");
    audio_vec
}

pub fn send_cover(path: &str) {
    // TODO: send cover via this
}
