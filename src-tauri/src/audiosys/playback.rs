use std::path::Path;

use ringbuf::{HeapRb, traits::Split};

pub fn play_audio(audio: &str) {
    let audio = Path::new(audio);
    let rb = HeapRb::<f64>::new(2);
    let (mut producer, mut consumer) = rb.split();
}
