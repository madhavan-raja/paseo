use clap::{Parser, Subcommand};
use crate::shell::SupportedShell;

/// A CLI tool to elegantly manage your shell's PATH variable.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Override the automatically detected shell
    #[arg(short, long, global = true)]
    pub shell: Option<SupportedShell>,

    /// Preview what would happen without making any changes to the file
    #[arg(short, long, global = true)]
    pub dry_run: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add a new directory to your managed paths
    Add {
        /// The directory path to add
        path: String,

        /// Verify that the directory actually exists on the filesystem before adding
        #[arg(short, long)]
        check: bool,
    },

    /// List all managed paths in alphabetical order
    List,

    /// Remove a path from the manager
    Remove {
        /// The directory path to remove. If omitted, opens an interactive selector.
        path: Option<String>,
    },

    /// Import paths into the manager
    Import {
        /// A raw PATH string (e.g., "dir1:dir2"). 
        /// If omitted, the app will attempt to read from STDIN, or fallback to the current $PATH.
        raw_path: Option<String>,
    },

    /// Output the managed paths as a single string formatted for your shell
    /// Usage (Bash/Zsh): export PATH="$(path-manager export)"
    Export,

    /// Generate tab-completion scripts for your shell
    GenerateCompletions {
        /// The shell to generate completions for
        shell: SupportedShell,
    },
}
