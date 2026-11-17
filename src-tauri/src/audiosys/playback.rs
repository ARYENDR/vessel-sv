use ringbuf::{HeapRb, traits::Split};

pub fn play_audio(audio: &str) {
    let rb = HeapRb::<f32>::new(48000 * 2);
    let (mut producer, mut consumer) = rb.split();
}
