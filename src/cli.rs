use std::path::PathBuf;

use clap::{Parser, Subcommand};
use crate::{detect_shell, shell::SupportedShell, store::PathStore};

/// A CLI tool to elegantly manage your shell's PATH variable.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Override the automatically detected shell
    #[arg(short, long, global = true, default_value = detect_shell().to_string())]
    pub shell: SupportedShell,

    /// Preview what would happen without making any changes to the pathfile
    #[arg(short, long, global = true)]
    pub dry_run: bool,

    /// Location of the pathfile
    #[arg(short, long, global = true, default_value = PathStore::default_file_path().into_os_string())]
    pub pathfile: PathBuf,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add a new directory to your managed paths
    Add {
        /// The directory path to add
        path: String,
    },

    /// List all managed paths
    List,

    /// Remove a path from the manager
    Remove {
        /// The directory path to remove.
        path: String,
    },

    /// Import paths into the manager
    Import {
        /// A raw PATH string (e.g., "dir1:dir2" for Bash). 
        /// If omitted, the app will attempt to read from STDIN, or fallback to the current $PATH.
        raw_path: Option<String>,
    },

    /// Output the managed paths as a single string formatted for a shell
    Export,

    /// Generate tab-completion scripts for your shell
    GenerateCompletions {
        /// The shell to generate completions for
        shell: SupportedShell,
    },
}
