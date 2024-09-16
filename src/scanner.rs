use walkdir::WalkDir;

pub fn scan_directory(dir: String) {
    let valid_files = vec!["mp3", "mp4"];

    for entry in WalkDir::new(dir) {
        let entry = entry.unwrap();
        if let Some(extension) = entry.path().extension() {
            if let Some(extension) = extension.to_str() {
                if valid_files.contains(&extension) {
                    println!("Found music file {:?}", entry.path());
                }
            }
        }
    }
}
