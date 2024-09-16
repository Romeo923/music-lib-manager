mod cli;
mod player;
mod scanner;

fn main() {
    let args = cli::parse_args();
    match args.command {
        cli::Command::Scan { directory } => {
            println!("Scanning {directory}...");
            scanner::scan_directory(directory)
        }
        cli::Command::List => println!("Listing Songs"),
        cli::Command::Play { file } => player::play_song(file),
    }
}
