mod cli;
mod scanner;

use cli::Command;

fn main() {
    let args = cli::parse_args();
    match args.command {
        Command::Scan { directory } => {
            println!("Scanning {directory}...");
            scanner::scan_directory(directory)
        },
        Command::List => println!("Listing Songs"),
        Command::Play { file } => println!("Now Playing {file}..."),
    }
}
