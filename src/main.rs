mod cli;

use cli::Command;

fn main() {
    let args = cli::parse_args();
    match args.command {
        Command::Scan { directory } => println!("Scanning {directory}"),
        Command::List => println!("Listing Songs"),
        Command::Play { file } => println!("Playing {file}"),
    }
}
