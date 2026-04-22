use std::path::PathBuf;

use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short, long, value_name = "FILE")]
    pub pathfile: Option<PathBuf>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List all directories in PATH
    List,
    /// Add to PATH
    Add(AddArgs)
}

#[derive(Args)]
pub struct AddArgs {
    pub directory: String
}
