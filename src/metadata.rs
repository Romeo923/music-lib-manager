use id3::{Tag, TagLike};
use serde::{Deserialize, Serialize};

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
    MusicFile {
        path: path.to_string(),
        title: tag.title().unwrap_or("Unknown").to_string(),
        artist: tag.artist().unwrap_or("Unknown").to_string(),
        album: tag.album().unwrap_or("Unknown").to_string(),
        genre: tag.genre().unwrap_or("Unknown").to_string(),
    }
}
