use id3::{Tag, TagLike};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct MusicFile {
    pub path: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub genre: String,
}

pub fn read_metadata(path: &str) -> MusicFile {
    let tag = Tag::read_from_path(path).unwrap();
    let file_name = Path::new(path)
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("Unknown");

    MusicFile {
        path: path.to_string(),
        title: tag.title().unwrap_or(file_name).to_string(),
        artist: tag.artist().unwrap_or("Unknown").to_string(),
        album: tag.album().unwrap_or("Unknown").to_string(),
        genre: tag.genre().unwrap_or("Unknown").to_string(),
    }
}
