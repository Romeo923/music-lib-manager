# Music Library Manager

A simple command-line music library manager built with Rust. This project scans a directory for `.mp3` files, reads their metadata (such as title, artist, album, and genre), and stores the information in a JSON file.

## Features

- Scans a directory for `.mp3` files.
- Extracts metadata (title, artist, album, and genre) using the `id3` crate.
- Automatically creates and updates a music library stored as a JSON file.
- Command-line interface (CLI) for scanning directories.

## Planned Features

- **Playlists**:
  - Create and manage custom playlists.
  - Add or remove songs from playlists.
  - Queue playlists

## Project Structure

```plaintext
.
├── src
│   ├── main.rs              # Main entry point for the application
│   ├── metadata.rs          # Logic for reading metadata
│   └── scan.rs              # Directory scanning and file management
├── Cargo.toml               # Rust dependencies and project metadata
└── README.md                # This file
```

## Dependencies
- [Rust](https://www.rust-lang.org/) (Latest stable version)
- [id3](https://crates.io/crates/id3): For reading MP3 metadata.
- [serde](https://crates.io/crates/serde): For serializing data structures.
- [serde_json](https://crates.io/crates/serde_json): For working with JSON data.
- [walkdir](https://crates.io/crates/walkdir): For recursive directory scanning.
