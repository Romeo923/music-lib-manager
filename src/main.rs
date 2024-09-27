mod cli;
mod music_library;

use clap::Parser;
use home::home_dir;
use music_library::MusicLibrary;

fn main() {
    let home = home_dir().expect("Unable ot find home directory");
    let lib_file = home.join(".local/share/music-lib-manager/music_library.json");
    let lib_file = lib_file.to_str().unwrap();

    let mut lib = match MusicLibrary::load_from_file(lib_file) {
        Ok(lib) => lib,
        Err(_) => MusicLibrary::new(),
    };

    let cli = cli::Cli::parse();

    match cli.command {
        cli::Commands::Play { name } => {
            println!("Playing: {name}!");
        }
        cli::Commands::Queue { name } => {
            println!("Queueing: {name}!");
        }
        cli::Commands::Scan { directory } => {
            lib.scan_directory(&directory);
        }
        cli::Commands::Songs { action } => match action {
            None => lib.list_songs(),
            Some(command) => match command {
                cli::SongAction::Add { path } => {
                    lib.add_song(path);
                }
                cli::SongAction::View { name } => {
                    lib.view_song(name);
                }
                cli::SongAction::Edit { name, field, value } => {
                    lib.edit_song(name, field, value);
                }
                cli::SongAction::Remove { name } => {
                    lib.remove_song(name);
                }
            },
        },
        cli::Commands::Playlists { action } => match action {
            None => lib.list_playlists(),
            Some(command) => match command {
                cli::PlaylistAction::Create { name } => {
                    lib.create_playlist(name);
                }
                cli::PlaylistAction::View { name } => {
                    lib.view_playlist(name);
                }
                cli::PlaylistAction::Add { name, song } => {
                    lib.add_song_playlist(name, song);
                }
                cli::PlaylistAction::Edit { name, field, value } => {
                    lib.edit_playist(name, field, value);
                }
                cli::PlaylistAction::Remove { name, song } => {
                    lib.remove_playlist_song(name, song);
                }
                cli::PlaylistAction::Delete { name } => {
                    lib.delete_playlist(name);
                }
            },
        },
    }

    match lib.save_to_file(lib_file) {
        Ok(_) => {}
        Err(_) => println!("Error while saving data"),
    };
}
