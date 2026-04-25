use std::path::PathBuf;

use clap::{Parser, Subcommand};
use crate::{detect_shell, shell::SupportedShell, store::PathStore};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// The shell for which the operations must be performed
    #[arg(short, long, global = true, default_value = detect_shell().to_string())]
    pub shell: SupportedShell,

    /// Location of the pathfile
    #[arg(short, long = "pathfile", global = true, default_value = PathStore::default_pathfilefile_path().into_os_string())]
    pub pathfile_location: PathBuf,

    /// Location of the pathfile backup
    #[arg(short, long = "backup", global = true, default_value = PathStore::default_backup_path().into_os_string())]
    pub backup_location: PathBuf,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add a new directory to PATH
    #[command(visible_aliases = ["new", "create"])]
    Add {
        /// The directory to add
        path: String,
    },

    /// Show all directories in PATH
    #[command(visible_aliases = ["list", "ls"])]
    Show {
        /// Format the output for the shell
        #[arg(short, long)]
        formatted: bool,
    },

    /// Remove a directory from PATH
    #[command(visible_aliases = ["delete", "del", "rm"])]
    Remove {
        /// The directory to remove
        path: String,
    },

    /// Import the directories from the shell's PATH
    Import {
        /// A raw PATH string (e.g., "dir1:dir2" for Bash). 
        /// If omitted, the app will attempt to read from STDIN, or fallback to the current $PATH.
        raw_path: Option<String>,

        /// Remove all existing directories before importing
        #[arg(short, long)]
        clear: bool,
    },

    /// Restore the directories from the pathfile backup
    Restore,

    /// Generate tab-completion scripts for your shell
    GenerateCompletions {
        /// The shell to generate completions for
        shell: SupportedShell,
    },
}
