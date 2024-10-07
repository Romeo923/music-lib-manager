use crate::music_library::{Playlist, Song};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::VecDeque;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader};

#[derive(Debug, Serialize, Deserialize)]
pub struct Queue {
    songs: VecDeque<Song>,
}

impl Queue {
    pub fn new() -> Self {
        Self {
            songs: VecDeque::new(),
        }
    }

    pub fn save_to_file(&self, file_name: &str) -> io::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(file_name)?;
        let _ = serde_json::to_writer_pretty(file, self);
        Ok(())
    }

    pub fn load_from_file(file_name: &str) -> io::Result<Self> {
        let file = File::open(file_name)?;
        let reader = BufReader::new(file);
        let queue: Queue = serde_json::from_reader(reader)?;
        Ok(queue)
    }

    pub fn add_song(&mut self, song: Song) {
        self.songs.push_back(song);
    }

    pub fn add_playlist(&mut self, playlist: Playlist) {
        for song in playlist.songs.iter() {
            self.add_song(song.clone());
        }
    }

    pub fn remove_song(&mut self, index: usize) -> Option<Song> {
        if let Some(song) = self.songs.remove(index) {
            Some(song)
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.songs.clear();
    }

    pub fn list(&self) {
        if self.songs.is_empty() {
            println!("The Queue is empty!");
        } else {
            for (i, song) in self.songs.iter().enumerate() {
                println!("{} - Song: {}", i, song.name.clone());
            }
        }
    }
}
