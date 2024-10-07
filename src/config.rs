use home::home_dir;
use std::path::PathBuf;

const DATA_DIR: &str = ".local/share/music-lib-manager/";

pub fn get_queue_file_path() -> PathBuf {
    let home = home_dir().expect("Unable to find home directory");
    home.join(DATA_DIR).join("queue.json")
}

pub fn get_library_file_path() -> PathBuf {
    let home = home_dir().expect("Unable to find home directory");
    home.join(DATA_DIR).join("music_library.json")
}
