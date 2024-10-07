mod cli;
mod music_library;
// mod player;
mod config;
mod queue;

use clap::Parser;
use music_library::MusicLibrary;
// use player::Player;
use queue::Queue;

fn main() {
    let mut lib = match MusicLibrary::load() {
        Ok(lib) => lib,
        Err(_) => MusicLibrary::new(),
    };

    let mut queue = match Queue::load() {
        Ok(queue) => queue,
        Err(_) => Queue::new(),
    };

    // load player rather than make a new one
    // similar to how lib is initialized
    // let mut player = Player::new();

    let cli = cli::Cli::parse();

    match cli.command {
        cli::Commands::Play => {
            // player.play();
        }
        cli::Commands::Pause => {
            // player.pause();
        }
        cli::Commands::Resume => {
            // player.resume();
        }
        cli::Commands::Stop => {
            // player.stop();
        }
        cli::Commands::Queue { action } => match action {
            None => queue.list(),
            Some(command) => match command {
                cli::QueueAction::AddSong { song_name } => {
                    if let Some(song) = lib.get_song(song_name.clone()) {
                        queue.add_song(song.clone());
                    } else {
                        println!("Song '{song_name}' not found in library.");
                    }
                }
                cli::QueueAction::AddPlaylist { playlist_name } => {
                    if let Some(playlist) = lib.get_playlist(playlist_name.clone()) {
                        queue.add_playlist(playlist.clone());
                    } else {
                        println!("Playlist '{playlist_name}' not found in library.");
                    }
                }
                cli::QueueAction::Remove { index } => {
                    queue.remove_song(index);
                }
                cli::QueueAction::List => {
                    queue.list();
                }
                cli::QueueAction::Clear => {
                    queue.clear();
                }
            },
        },
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
                cli::SongAction::List => {
                    lib.list_songs();
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
                cli::PlaylistAction::List => {
                    lib.list_playlists();
                }
            },
        },
    }

    match lib.save() {
        Ok(_) => {}
        Err(_) => println!("Error while saving data"),
    };

    match queue.save() {
        Ok(_) => {}
        Err(_) => println!("Error while saving data"),
    };
}
