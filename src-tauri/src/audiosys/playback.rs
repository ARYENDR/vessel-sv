use std::{
    fs::File,
    io::BufReader,
    sync::{Mutex, mpsc},
    thread,
};

use rodio::{Decoder, DeviceSinkBuilder, Player};

enum Message {
    Play(String),
    Pause,
    Resume,
    Stop,
}

pub struct AudTx(Mutex<mpsc::Sender<Message>>);

pub fn spawn_audio_thread() -> AudTx {
    let (tx, rx) = mpsc::channel::<Message>();

    thread::spawn(move || {
        let sink = DeviceSinkBuilder::open_default_sink().expect("fail to open sink");
        let player = Player::connect_new(&sink.mixer());

        while let Ok(msg) = rx.recv() {
            match msg {
                Message::Play(path) => {
                    let file = match File::open(&path) {
                        Ok(f) => f,
                        Err(e) => {
                            println!("can't open: {}", e);
                            continue;
                        }
                    };
                    match Decoder::try_from(BufReader::new(file)) {
                        Ok(source) => {
                            player.clear();
                            player.append(source);
                            player.play();
                        }
                        Err(e) => println!("can't decode: {}", e),
                    }
                }
                Message::Pause => player.pause(),
                Message::Resume => player.play(),
                Message::Stop => player.clear(),
            }
        }
    });

    AudTx(Mutex::new(tx))
}
