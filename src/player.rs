use daemonize::Daemonize;
use rodio::{Decoder, OutputStream, Sink};
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
    Starting,
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

pub struct Player;

impl Player {
    pub fn play() {
        PlayerState::Starting.save();

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
                            break;
                        }
                    };

                    match state {
                        PlayerState::Starting | PlayerState::Playing(_) => {
                            if sink.is_paused() {
                                sink.play();
                            } else if sink.empty() {
                                match Queue::load() {
                                    Ok(mut queue) => {
                                        if let Some((song, source)) = get_next_song_data(&mut queue)
                                        {
                                            sink.append(source);
                                            if let Ok(_) = queue.save() {
                                                PlayerState::Playing(song).save();
                                            } else {
                                                eprintln!("Failed to update queue");
                                            }
                                        } else {
                                            sink.stop();
                                            PlayerState::Stopped.save();
                                            break;
                                        }
                                    }
                                    Err(_) => {
                                        eprintln!("Failed to load queue.");
                                        PlayerState::Stopped.save();
                                        break;
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
                            if !sink.is_paused() {
                                sink.stop();
                            }

                            match Queue::load() {
                                Ok(mut queue) => {
                                    if let Some((song, source)) = get_next_song_data(&mut queue) {
                                        sink.append(source);
                                        if let Ok(_) = queue.save() {
                                            PlayerState::Playing(song).save();
                                        } else {
                                            eprintln!("Failed to update queue");
                                        }
                                    }
                                }
                                Err(_) => {
                                    eprintln!("Failed to load queue.");
                                    PlayerState::Stopped.save();
                                    break;
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

    pub fn pause() {
        let state = PlayerState::load().unwrap();
        if let PlayerState::Playing(song) = state {
            PlayerState::Paused(song).save();
        }
    }

    pub fn resume() {
        let state = PlayerState::load().unwrap();
        if let PlayerState::Paused(song) = state {
            PlayerState::Playing(song).save();
        }
    }

    pub fn stop() {
        PlayerState::Stopped.save();
    }

    pub fn skip() {
        let state = PlayerState::load().unwrap();
        if let PlayerState::Playing(song) | PlayerState::Paused(song) = state {
            PlayerState::Skip(song).save();
        }
    }

    pub fn current_song() -> Option<Song> {
        if let Ok(state) = PlayerState::load() {
            match state {
                PlayerState::Stopped => None,
                PlayerState::Skip(ref song) => Some(song.clone()),
                PlayerState::Paused(ref song) => Some(song.clone()),
                PlayerState::Playing(ref song) => Some(song.clone()),
                PlayerState::Starting => None,
            }
        } else {
            eprintln!("Failed to load queue.");
            PlayerState::Stopped.save();
            None
        }
    }
}

fn get_next_song_data(queue: &mut Queue) -> Option<(Song, Decoder<BufReader<File>>)> {
    let song = queue.pop()?;
    let file = File::open(&song.path).ok()?;
    let source = Decoder::new(BufReader::new(file)).ok()?;
    Some((song, source))
}
