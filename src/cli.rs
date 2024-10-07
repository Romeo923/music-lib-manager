use clap::{Parser, Subcommand};

/// Music Library Manager CLI
#[derive(Parser, Debug)]
#[command(name = "Music Library Manager")]
#[command(version, about = "Manage your music library and playlists", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Play all songs in the queue
    Play,
    /// Pause playback
    Pause,
    /// Resume playback
    Resume,
    /// Stop playback
    Stop,
    /// Add a song or playlist to the queue
    Queue {
        #[clap(subcommand)]
        action: Option<QueueAction>,
    },
    /// Scan a directory for music files
    Scan {
        /// Directory path
        directory: String,
    },
    /// Add, Edit, Remove, or List songs
    Songs {
        #[clap(subcommand)]
        action: Option<SongAction>,
    },
    /// Create, Edit, Remove, or List playlists
    Playlists {
        #[clap(subcommand)]
        action: Option<PlaylistAction>,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum SongAction {
    /// Add a song
    Add {
        path: String,
    },
    /// View song details
    View {
        name: String,
    },
    /// Remove a song
    Remove {
        name: String,
    },
    /// Edit a song
    Edit {
        /// Song Name
        name: String,
        #[clap(subcommand)]
        /// Name, Artist, Album, or Path
        field: EditSong,
        /// New Value
        value: String,
    },
    List,
}

#[derive(Debug, Clone, Subcommand)]
pub enum EditSong {
    Name,
    Artist,
    Album,
    Path,
}

#[derive(Subcommand, Debug, Clone)]
pub enum PlaylistAction {
    /// Create a playlist
    Create {
        name: String,
    },
    /// View playlist details
    View {
        name: String,
    },
    /// Add a song to playlist
    Add {
        name: String,
        song: String,
    },
    /// Remove a song from a playlist
    Remove {
        name: String,
        song: String,
    },
    /// Delete a playlist
    Delete {
        name: String,
    },
    /// Edit a playlist
    Edit {
        /// Playlist Name
        name: String,
        #[clap(subcommand)]
        /// Name
        field: EditPlaylist,
        /// New Value
        value: String,
    },
    List,
}

#[derive(Debug, Clone, Subcommand)]
pub enum EditPlaylist {
    Name,
}

#[derive(Debug, Clone, Subcommand)]
pub enum QueueAction {
    AddSong { song_name: String },
    AddPlaylist { playlist_name: String },
    Remove { index: usize },
    List,
    Clear,
}
