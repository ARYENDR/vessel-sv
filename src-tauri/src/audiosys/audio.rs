/// # Audio: Invoked
/// Each audio file will be stored as one of these structs before being sent to J*vaScript:
pub struct Audio {
    title: String,
    album: String,
    artists: Vec<String>,
    track_number: u16,
    path: String,
    cover: String, // idk what this is supposed to be
    length: u32,   // in milliseconds
    lyric_path: String,
    description: String,
    rating: u8, // 0-10
}
