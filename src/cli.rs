use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Command {
    Scan { directory: String },
    List,
    Play { file: String },
}

#[derive(StructOpt)]
pub struct Cli {
    #[structopt(subcommand)]
    pub command: Command,
}

pub fn parse_args() -> Cli {
    Cli::from_args()
}
