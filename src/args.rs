use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Parser)]
pub struct EncodeArgs {
    pub file_path: String,

    pub chunk_type: String,
    pub message: String,

    pub output_file: Option<String>,
}

#[derive(Parser)]
pub struct DecodeArgs {
    pub file_path: String,

    pub chunk_type: String,
}

#[derive(Parser)]
pub struct RemoveArgs {
    pub file_path: String,

    pub chunk_type: String,
}

#[derive(Parser)]
pub struct PrintArgs {
    pub file_path: String,
}
