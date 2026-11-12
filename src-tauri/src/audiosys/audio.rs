use serde::Serialize;

/// # Audio: Invoked
/// Each audio file will be stored as one of these structs before being sent to J*vaScript:
#[derive(Serialize)]
pub struct VesAudio {
    title: String,
    album: String,
    artists: Vec<String>,
    track_number: u16,
    path: String,
    cover: String, // idk what this is supposed to be
    length: f64,   // in milliseconds
    lyric_path: String,
    description: String,
    rating: u8, // 0-10
}

impl VesAudio {
    pub fn new(
        title: String,
        album: String,
        artists: Vec<String>,
        track_number: u16,
        path: String,
        cover: String,
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
            cover,
            length,
            lyric_path,
            description,
            rating,
        }
    }
}
