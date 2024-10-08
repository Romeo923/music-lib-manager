use daemonize::Daemonize;
use rodio::{OutputStream, Sink};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader};
use std::{process, thread, time::Duration};

use crate::config;
use crate::music_library::Song;
use crate::queue::Queue;

#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayerState {
    Stopped,
    Skip(Song),
    Paused(Song),
    Playing(Song),
}

impl PlayerState {
    pub fn load() -> io::Result<Self> {
        let file_name = config::get_player_file_path();
        let file = File::open(file_name)?;
        let reader = BufReader::new(file);
        let state: PlayerState = serde_json::from_reader(reader)?;
        Ok(state)
    }

    pub fn save(&self) -> io::Result<()> {
        let file_name = config::get_player_file_path();
        if let Some(parent) = file_name.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(file_name)?;
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }
}

struct Playing {
    song: Song,
}
struct Paused {
    song: Song,
}
struct Stopped;

pub trait PlayerAction {
    fn play(&self);
    fn pause(&self);
    fn resume(&self);
    fn skip(&self);
    fn stop(&self);
    fn status(&self);
}

impl PlayerAction for Playing {
    fn play(&self) {
        println!(
            "{} by {} is already playing",
            self.song.name, self.song.artist
        );
    }
    fn pause(&self) {
        let _ = PlayerState::Paused(self.song.clone()).save();
    }
    fn resume(&self) {
        println!(
            "{} by {} is already playing",
            self.song.name, self.song.artist
        );
    }
    fn stop(&self) {
        let _ = PlayerState::Stopped.save();
    }
    fn skip(&self) {
        let _ = PlayerState::Skip(self.song.clone()).save();
    }
    fn status(&self) {
        println!("Playing: {} by {}", self.song.name, self.song.artist);
    }
}

impl PlayerAction for Paused {
    fn play(&self) {
        self.resume();
    }
    fn pause(&self) {
        println!(
            "{} by {} is already paused",
            self.song.name, self.song.artist
        );
    }
    fn resume(&self) {
        let _ = PlayerState::Playing(self.song.clone()).save();
    }
    fn stop(&self) {
        let _ = PlayerState::Stopped.save();
    }
    fn skip(&self) {
        let _ = PlayerState::Skip(self.song.clone()).save();
    }
    fn status(&self) {
        println!("Paused: {} by {}", self.song.name, self.song.artist);
    }
}

impl PlayerAction for Stopped {
    fn play(&self) {
        match Queue::load() {
            Ok(queue) => match queue.peek() {
                Some(song) => {
                    let _ = PlayerState::Playing(song.clone()).save();
                    create_daemon();
                }
                None => println!("Queue is empty!"),
            },
            Err(_) => println!("Error loading queue"),
        }
    }
    fn pause(&self) {
        println!("No song playing");
    }
    fn resume(&self) {
        println!("No song playing");
    }
    fn stop(&self) {
        println!("No song playing");
    }
    fn skip(&self) {
        println!("No song playing");
    }
    fn status(&self) {
        println!("No songs playing");
    }
}

pub struct Player;

impl Player {
    pub fn load() -> Box<dyn PlayerAction> {
        let player: Box<dyn PlayerAction> = match PlayerState::load() {
            Ok(state) => match state {
                PlayerState::Playing(song) => Box::new(Playing { song }),
                PlayerState::Paused(song) => Box::new(Paused { song }),
                PlayerState::Skip(song) => Box::new(Playing { song }),
                PlayerState::Stopped => Box::new(Stopped),
            },
            Err(_) => {
                let _ = PlayerState::Stopped.save();
                Box::new(Stopped)
            }
        };
        player
    }
}

fn create_daemon() {
    let stdout = File::create("/tmp/music-lib-player.out").unwrap();
    let stderr = File::create("/tmp/music-lib-player.err").unwrap();

    let daemonize = Daemonize::new()
        .pid_file("/tmp/music-lib-player.pid")
        .chown_pid_file(true)
        .working_directory("/tmp")
        .stdout(stdout)
        .stderr(stderr)
        .privileged_action(move || {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();

            loop {
                let state = match PlayerState::load() {
                    Ok(state) => state,
                    Err(_) => {
                        eprintln!("Error loading player state");
                        process::exit(1);
                    }
                };

                match state {
                    PlayerState::Playing(_) => {
                        if sink.is_paused() {
                            // resume if simk is paused
                            sink.play();
                        } else if sink.empty() {
                            // if sink is empty, play next song in queue
                            match Queue::load() {
                                Ok(mut queue) => match queue.pop() {
                                    Some(song) => match song.get_source() {
                                        Some(source) => {
                                            sink.append(source);
                                            if let Ok(_) = queue.save() {
                                                let _ = PlayerState::Playing(song).save();
                                            } else {
                                                eprintln!("Failed to update queue");
                                            }
                                        }
                                        None => {
                                            // unable to play song
                                            let _ = PlayerState::Skip(song).save();
                                        }
                                    },
                                    None => {
                                        // Queue is empty
                                        sink.stop();
                                        let _ = PlayerState::Stopped.save();
                                        break;
                                    }
                                },
                                Err(_) => {
                                    eprintln!("Failed to load queue.");
                                    let _ = PlayerState::Stopped.save();
                                    process::exit(1);
                                }
                            }
                        }
                    }
                    PlayerState::Paused(_) => {
                        if !sink.is_paused() {
                            sink.pause();
                        }
                    }
                    PlayerState::Skip(_) => {
                        sink.stop();
                        // play next song in queue
                        match Queue::load() {
                            Ok(mut queue) => match queue.pop() {
                                Some(song) => {
                                    if let Some(source) = song.get_source() {
                                        sink.append(source);
                                        if let Ok(_) = queue.save() {
                                            let _ = PlayerState::Playing(song).save();
                                        } else {
                                            eprintln!("Failed to update queue");
                                        }
                                    } else {
                                        // if we are unable to get source,
                                        // save queue and keep skipping
                                        if let Ok(_) = queue.save() {
                                            let _ = PlayerState::Skip(song).save();
                                        } else {
                                            eprintln!("Failed to update queue");
                                        }
                                    }
                                }
                                None => {
                                    // queue is empty
                                    let _ = PlayerState::Stopped.save();
                                }
                            },
                            Err(_) => {
                                eprintln!("Failed to load queue.");
                                let _ = PlayerState::Stopped.save();
                                process::exit(1);
                            }
                        }
                    }
                    PlayerState::Stopped => {
                        if !sink.empty() {
                            sink.stop();
                            break;
                        }
                    }
                }
                thread::sleep(Duration::from_secs(1));
            }
            process::exit(0);
        });
    match daemonize.start() {
        Ok(_) => println!("Starting Playback"),
        Err(_) => println!("Error Starting Playback"),
    }
}
