use clap::StructOpt;

use pngme::{
    args::{Cli, Command},
    commands, Result,
};

fn main() -> Result<()> {
    match &Cli::parse().command {
        Command::Encode(args) => commands::encode(args),
        Command::Decode(args) => commands::decode(args),
        Command::Remove(args) => commands::remove(args),
        Command::Print(args) => commands::print(args),
    }
}
