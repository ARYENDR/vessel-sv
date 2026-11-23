use std::{
    fs::File,
    io::BufReader,
    sync::{Mutex, mpsc},
    thread,
    time::Duration,
};

use rodio::{Decoder, DeviceSinkBuilder, Player};
use tauri::State;

enum Message {
    Play(String),
    Pause,
    Resume,
    Stop,
}


// this took me days of documentation and i am proud of it aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
pub struct AudTx(Mutex<mpsc::Sender<Message>>);

pub fn spawn_audio_thread() -> AudTx {
    let (tx, rx) = mpsc::channel::<Message>();

    thread::spawn(move || {
        let sink = DeviceSinkBuilder::open_default_sink().expect("fail to open sink");
        let player = Player::connect_new(&sink.mixer());

        while let Ok(msg) = rx.recv() {
            match msg {
                Message::Play(path) => {
                    player.set_volume(1.0);
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
                Message::Pause => {
                    while player.volume() > 0.0 {
                        player.set_volume(player.volume() - 0.01);
                        thread::sleep(Duration::from_millis(5));
                    }
                    player.pause();
                    player.set_volume(1.0);
                }
                Message::Resume => {
                    player.set_volume(0.0);
                    player.play();
                    while player.volume() < 1.0 {
                        player.set_volume(player.volume() + 0.01);
                        thread::sleep(Duration::from_millis(2));
                    }
                    player.set_volume(1.0);
                }
                Message::Stop => player.clear(),
            }
        }
    });

    AudTx(Mutex::new(tx))
}

#[tauri::command]
pub fn playback(
    message: String,
    path: Option<String>,
    audio: State<'_, AudTx>,
) -> Result<(), String> {
    let cmd = match message.as_str() {
        "Play" => Message::Play(path.ok_or("Play needs a path")?),
        "Pause" => Message::Pause,
        "Resume" => Message::Resume,
        "Stop" => Message::Stop,
        other => return Err(format!("unknown playback message: {other}")),
    };
    audio.0.lock().unwrap().send(cmd).map_err(|e| e.to_string())
}
