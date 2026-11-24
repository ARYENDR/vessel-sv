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
    PlayPause,
    Stop,
    Seek(f64),
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
                    match Decoder::try_from(file) {
                        Ok(source) => {
                            player.clear();
                            player.append(source);
                            player.play();
                        }
                        Err(e) => println!("can't decode: {}", e),
                    }
                }
                Message::PlayPause => {
                    match player.is_paused() {
                        false => {
                            while player.volume() > 0.0 {
                                player.set_volume(player.volume() - 0.01);
                                thread::sleep(Duration::from_millis(5));
                            }
                            player.pause();
                            player.set_volume(1.0);
                        }
                        true => {
                            player.set_volume(0.0);
                            player.play();
                            while player.volume() < 1.0 {
                                player.set_volume(player.volume() + 0.01);
                                thread::sleep(Duration::from_millis(5));
                            }
                            player.set_volume(1.0);
                        }
                    }
                    println!("{}", player.is_paused());
                }
                Message::Stop => player.clear(),
                Message::Seek(d) => {
                    if d.is_sign_negative() {
                        let d = d * -1.0;
                        if player.get_pos().clone().as_secs_f64() - d < 0.0 {
                            player.try_seek(Duration::from_secs_f64(0.0)).unwrap();
                        } else {
                            player
                                .try_seek(player.get_pos() - Duration::from_secs_f64(d))
                                .unwrap();
                        }
                    } else {
                        player
                            .try_seek(player.get_pos() + Duration::from_secs_f64(d))
                            .unwrap();
                    }
                    println!("{}", player.get_pos().as_secs_f64());
                }
            }
        }
    });

    AudTx(Mutex::new(tx))
}

#[tauri::command]
pub fn playback(
    message: String,
    path: Option<String>,
    duration: Option<f64>,
    audio: State<'_, AudTx>,
) -> Result<(), String> {
    let cmd = match message.as_str() {
        "Play" => Message::Play(path.ok_or("Play needs a path")?),
        "PlayPause" => Message::PlayPause,
        "Stop" => Message::Stop,
        "Seek" => Message::Seek(duration.ok_or(0.0).unwrap()),
        other => return Err(format!("unknown playback message: {other}")),
    };
    audio.0.lock().unwrap().send(cmd).map_err(|e| e.to_string())
}
