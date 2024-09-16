use crate::metadata::{read_metadata, MusicFile};
use std::fs::{create_dir_all, OpenOptions};
use std::path::Path;
use walkdir::WalkDir;

pub fn scan_directory(dir: String, lib_file: &str) {
    let valid_files = vec!["mp3", "mp4"];
    let mut library: Vec<MusicFile> = Vec::new();

    for entry in WalkDir::new(dir) {
        let entry = entry.unwrap();
        if let Some(extension) = entry.path().extension() {
            if let Some(extension) = extension.to_str() {
                if valid_files.contains(&extension) {
                    let music_file = read_metadata(entry.path().to_str().unwrap());
                    library.push(music_file);
                }
            }
        }
    }

    let parent_dir = Path::new(lib_file).parent().unwrap();
    if !parent_dir.exists() {
        create_dir_all(parent_dir).unwrap();
    }

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(lib_file)
        .expect(&format! {"Unable to open file: {lib_file}"});

    serde_json::to_writer_pretty(file, &library).unwrap();

    println!("Scanning Complete!");
}
