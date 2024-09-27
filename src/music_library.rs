use crate::cli::{EditPlaylist, EditSong};
use clap::Parser;
use lofty::file::{AudioFile, TaggedFileExt};
use lofty::probe::Probe;
use lofty::tag::ItemKey;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
    pub name: String,
    pub artist: String,
    pub album: String,
    pub duration: u64, //seconds
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    pub name: String,
    pub songs: Vec<Song>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicLibrary {
    pub songs: HashMap<String, Song>,
    pub playlists: HashMap<String, Playlist>,
}

impl MusicLibrary {
    pub fn new() -> Self {
        MusicLibrary {
            songs: HashMap::new(),
            playlists: HashMap::new(),
        }
    }

    pub fn save_to_file(&self, file_name: &str) -> io::Result<()> {
        let file = File::create(file_name)?;
        let json_data = serde_json::to_string_pretty(self)?;
        let mut writer = io::BufWriter::new(file);
        writer.write_all(json_data.as_bytes())?;
        Ok(())
    }

    pub fn load_from_file(file_name: &str) -> io::Result<Self> {
        let mut file = File::open(file_name)?;
        let mut json_data = String::new();
        file.read_to_string(&mut json_data)?;
        let library: MusicLibrary = serde_json::from_str(&json_data)?;
        Ok(library)
    }

    pub fn scan_directory(&mut self, directory: &String) {
        for entry in WalkDir::new(directory)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| is_music_file(e.path()))
        {
            let path = entry.path();

            if !is_music_file(&path) {
                continue;
            }

            if self
                .songs
                .values()
                .any(|song| song.path == path.to_string_lossy().to_string())
            {
                continue;
            }

            if let Some(song) = read_metadata(path) {
                self.songs.insert(song.name.clone(), song);
            }
        }
    }

    // SONGS

    pub fn list_songs(&self) {
        self.songs
            .iter()
            .for_each(|(song_name, _)| println!("{}", song_name));
    }

    pub fn add_song(&mut self, path: String) {
        let path = Path::new(&path);

        if let Some(song) = read_metadata(path) {
            if self.songs.contains_key(&song.name) {
                println!("Song '{}' already exists in the library.", song.name);
            } else {
                self.songs.insert(song.name.clone(), song);
            }
        }
    }

    pub fn view_song(&self, name: String) {
        if let Some(song) = self.songs.get(&name) {
            println!("Name:      {}", song.name);
            println!("Artist:    {}", song.artist);
            println!("Album:     {}", song.album);
            println!("Duration:  {} seconds", song.duration);
            println!("File Path: {}", song.path);
        } else {
            println!("'{}' not found", name);
        }
    }

    pub fn edit_song(&mut self, name: String, field: EditSong, value: String) {
        if let Some(song) = self.songs.get_mut(&name) {
            match field {
                EditSong::Name => song.name = value,
                EditSong::Artist => song.artist = value,
                EditSong::Album => song.album = value,
                EditSong::Path => song.path = value,
            }
        } else {
            println!("Song '{name} not found")
        }
    }

    pub fn remove_song(&mut self, name: String) {
        if self.songs.remove(&name).is_some() {
            println!("Removed {name} from library");
        } else {
            println!("{name} does not exist in library");
        }
    }

    // PLAYLISTS

    pub fn list_playlists(&self) {
        self.playlists
            .iter()
            .for_each(|(playlist_name, _)| println!("{}", playlist_name));
    }

    pub fn create_playlist(&mut self, name: String) {
        self.playlists.insert(
            name.clone(),
            Playlist {
                name,
                songs: Vec::new(),
            },
        );
    }

    pub fn view_playlist(&self, name: String) {
        if let Some(playlist) = self.playlists.get(&name) {
            println!("Name:  {}", playlist.name);
            println!("Songs:");
            playlist
                .songs
                .iter()
                .for_each(|song| println!("       {0} - {1}", song.name, song.artist));
        } else {
            println!("'{}' not found", name);
        }
    }

    pub fn add_song_playlist(&mut self, name: String, song_name: String) {
        if let Some(playlist) = self.playlists.get_mut(&name) {
            if let Some(song) = self.songs.get(&song_name) {
                playlist.songs.push(song.clone());
            } else {
                println!("{song_name} does not exist in library.");
            }
        } else {
            println!("'{}' not found", name);
        }
    }

    pub fn edit_playist(&mut self, name: String, field: EditPlaylist, value: String) {
        if let Some(playlist) = self.playlists.get_mut(&name) {
            match field {
                EditPlaylist::Name => playlist.name = value,
            }
        } else {
            println!("Song '{name} not found")
        }
    }

    pub fn remove_playlist_song(&mut self, name: String, song_name: String) {
        if let Some(playlist) = self.playlists.get_mut(&name) {
            if let Some(index) = playlist
                .songs
                .iter()
                .position(|song| song.name == song_name)
            {
                playlist.songs.swap_remove(index);
                println!("Removed {song_name} from {name}");
            } else {
                println!("{song_name} does not exist in {name}");
            }
        } else {
            println!("Playlist '{name}' does not exist");
        }
    }

    pub fn delete_playlist(&mut self, name: String) {
        if self.playlists.remove(&name).is_some() {
            println!("Deleted playlist '{name}'");
        } else {
            println!("Playlist '{name}' does not exist.");
        }
    }
}

fn read_metadata(path: &Path) -> Option<Song> {
    let tagged_file = Probe::open(path).ok()?.read().ok()?;

    let file_name = path.file_stem()?.to_string_lossy().to_string();
    let file_path = path.to_string_lossy().to_string();

    let properties = tagged_file.properties();
    let duration = properties.duration().as_secs();

    let tag = tagged_file.primary_tag();
    Some(Song {
        name: tag?
            .get_string(&ItemKey::TrackTitle)
            .unwrap_or(&file_name)
            .to_string(),
        artist: tag?
            .get_string(&ItemKey::TrackArtist)
            .unwrap_or("Unknown")
            .to_string(),
        album: tag?
            .get_string(&ItemKey::AlbumTitle)
            .unwrap_or("Unknown")
            .to_string(),
        duration,
        path: file_path,
    })
}

fn is_music_file(path: &Path) -> bool {
    let valid_exts = vec!["mp3", "flac", "wav", "ogg"];
    if let Some(ext) = path.extension() {
        if let Some(ext) = ext.to_str() {
            return valid_exts.contains(&ext);
        }
    }
    false
}
