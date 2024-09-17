use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(PartialEq, Eq)]
pub enum PlayerState {
    Stopped,
    Paused,
    Playing,
}

pub struct Player {
    state: PlayerState,
    sink: Arc<Mutex<Option<Sink>>>,
    stream: Option<OutputStream>,
}

impl Player {
    pub fn new() -> Player {
        Player {
            state: PlayerState::Stopped,
            sink: Arc::new(Mutex::new(None)),
            stream: None,
        }
    }

    pub fn play(&mut self, path: String) {
        if self.state != PlayerState::Stopped {
            return;
        }

        let sink = Arc::clone(&self.sink);
        let (stream, stream_handle) = OutputStream::try_default().unwrap();

        thread::spawn(move || {
            let file = File::open(path).unwrap();
            let source = Decoder::new(BufReader::new(file)).unwrap();

            let new_sink = Sink::try_new(&stream_handle).unwrap();
            new_sink.append(source);

            *sink.lock().unwrap() = Some(new_sink);
        });

        self.stream = Some(stream);
        self.state = PlayerState::Playing;
    }

    pub fn pause(&mut self) {
        if self.state != PlayerState::Playing {
            return;
        }

        if let Some(ref sink) = *self.sink.lock().unwrap() {
            sink.pause();
        }

        self.state = PlayerState::Paused;
    }

    pub fn resume(&mut self) {
        if self.state != PlayerState::Paused {
            return;
        }

        if let Some(ref sink) = *self.sink.lock().unwrap() {
            sink.play();
        }

        self.state = PlayerState::Playing;
    }

    pub fn stop(&mut self) {
        if self.state == PlayerState::Stopped {
            return;
        }

        if let Some(ref sink) = *self.sink.lock().unwrap() {
            sink.stop();
        }

        self.stream = None;
        self.state = PlayerState::Stopped;
    }
}
