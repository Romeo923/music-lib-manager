mod cli;
mod metadata;
mod player;
mod scanner;

use home::home_dir;

fn main() {
    let home = home_dir().expect("Unable ot find home directory");
    let lib_file = home.join(".local/share/music-lib-manager/music_library.json");

    let lib_file = lib_file.to_str().unwrap();

    let args = cli::parse_args();
    match args.command {
        cli::Command::Scan { directory } => {
            println!("Scanning {directory}...");
            scanner::scan_directory(directory, lib_file)
        }
        cli::Command::List => println!("Listing Songs"),
        cli::Command::Play { file } => player::play_song(file),
    }
}
