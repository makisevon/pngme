use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Debug, Parser)]
pub struct EncodeArgs {
    pub file_path: String,
    pub chunk_type: String,
    pub message: String,
    pub output_file: Option<String>,
}

#[derive(Debug, Parser)]
pub struct DecodeArgs {
    pub file_path: String,
    pub chunk_type: String,
}

#[derive(Debug, Parser)]
pub struct RemoveArgs {
    pub file_path: String,
    pub chunk_type: String,
}

#[derive(Debug, Parser)]
pub struct PrintArgs {
    pub file_path: String,
}
