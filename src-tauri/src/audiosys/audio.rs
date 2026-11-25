use lofty::{
    file::{AudioFile, TaggedFileExt},
    tag::Accessor,
};
use serde::Serialize;

/// # Audio: Invoked
/// Each audio file will be stored as one of these structs before being sent to J*vaScript:
#[derive(Serialize, Clone)]
pub struct VesAudio {
    title: String,
    album: String,
    artists: Vec<String>,
    track_number: u32,
    path: String,
    has_cover: bool,
    length: f64, // in milliseconds
    lyric_path: String,
    description: String,
    rating: u8, // 0-10
}

impl VesAudio {
    pub fn new(
        title: String,
        album: String,
        artists: Vec<String>,
        track_number: u32,
        path: String,
        has_cover: bool,
        length: f64,
        lyric_path: String,
        description: String,
        rating: u8,
    ) -> VesAudio {
        VesAudio {
            title,
            album,
            artists,
            track_number,
            path,
            has_cover,
            length,
            lyric_path,
            description,
            rating,
        }
    }

    pub fn get(path: &str) -> VesAudio {
        let tag = lofty::read_from_path(path).unwrap();
        let tagged = tag.primary_tag().unwrap();
        let audiotagged = audiotags::Tag::new().read_from_path(path).unwrap();
        VesAudio {
            title: tagged.title().unwrap().to_string(),
            album: tagged.album().unwrap().to_string(),
            artists: audiotagged
                .artists()
                .unwrap()
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
            track_number: tagged.track().unwrap(),
            path: path.to_string(),
            has_cover: tagged.pictures().is_empty(),
            length: tag.properties().duration().as_secs_f64(),
            lyric_path: path.clone().replace("mp3", "lrc"),
            description: "no desc".to_string(),
            rating: 7,
        }
    }
}
