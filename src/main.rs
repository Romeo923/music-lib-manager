mod metadata;
mod player;
mod scanner;

use home::home_dir;
use player::Player;

fn main() {
    let home = home_dir().expect("Unable ot find home directory");
    let lib_file = home.join(".local/share/music-lib-manager/music_library.json");
    let lib_file = lib_file.to_str().unwrap();

    let mut my_player = Player::new();

    loop {
        println!("\nEnter command:\n  scan <dir>\n  list\n  play <file_path>\n  pause\n  resume\n  stop\n  exit | quit | q");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let mut words = input.trim().split_whitespace();
        let command = words.next();
        let argument = words.next();

        match command {
            Some("scan") => {
                if let Some(dir) = argument {
                    scanner::scan_directory(dir.to_string(), lib_file);
                }
            }
            Some("list") => {
                scanner::list_songs(lib_file);
            }
            Some("play") => {
                if let Some(file_path) = argument {
                    my_player.play(file_path.to_string());
                }
            }
            Some("pause") => {
                my_player.pause();
            }
            Some("resume") => {
                my_player.resume();
            }
            Some("stop") => {
                my_player.stop();
            }
            Some("exit") => {
                return;
            }
            Some("quit") => {
                return;
            }
            Some("q") => {
                return;
            }
            _ => {
                println!("Invalid Command");
            }
        }
    }
}
