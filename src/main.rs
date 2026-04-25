mod cli;
mod shell;
mod store;

use anyhow::{Context, Result};
use clap::{CommandFactory, Parser};
use std::env;
use std::io::{self, IsTerminal, Read};
use std::path::{Path, PathBuf};

use cli::{Cli, Commands};
use shell::SupportedShell;
use store::PathStore;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let shell_type = cli.shell;
    let shell_impl = shell_type.build();

    let mut store =
        PathStore::load(cli.pathfile_location, cli.backup_location).context("Failed to load paths from storage file")?;

    match cli.command {
        Commands::Add {
            path,
        } => {
            let expanded_path = get_absolute_path(&path)?;

            if store.insert(expanded_path.clone()) {
                store.save()?;
                println!("Added '{}'", expanded_path);
            } else {
                println!("Directory '{}' is already in pathfile.", expanded_path);
            }
        }

        Commands::Show { formatted } => {
            let paths = store.get_all();

            if formatted {
                let export_string = shell_impl.generate_shell_path(&paths);
                print!("{}", export_string);
            } else {
                if paths.is_empty() {
                    println!("No directory currently in pathfile.");
                } else {
                    for path in paths {
                        println!("{}", path);
                    }
                }
            }
        }

        Commands::Remove { path } => {
            let expanded_path = get_absolute_path(&path)?;

            if store.remove(&expanded_path) {
                store.save()?;
                println!("Removed '{}'", expanded_path);
            } else {
                println!("Directory '{}' is not in pathfile.", expanded_path);
            }
        }

        Commands::Import { raw_path, clear } => {
            if clear {
                store.clear();
            }

            let path_string = resolve_import_string(raw_path)?;
            let parsed_paths = shell_impl.parse_shell_path(&path_string);

            let mut added_count = 0;
            for p in parsed_paths {
                if store.insert(p) {
                    added_count += 1;
                }
            }

            if added_count > 0 {
                store.save()?;
                println!("Imported {} new directories.", added_count);
            } else {
                println!("No new directories found.");
            }
        }

        Commands::Restore => {
            store.restore()?;
        }

        Commands::GenerateCompletions {
            shell: target_shell,
        } => {
            let mut cmd = Cli::command();
            let bin_name = cmd.get_name().to_string();

            let clap_shell = match target_shell {
                SupportedShell::Bash => clap_complete::Shell::Bash,
                SupportedShell::Zsh => clap_complete::Shell::Zsh,
                SupportedShell::Fish => clap_complete::Shell::Fish,
                SupportedShell::Nu => clap_complete::Shell::Elvish,
            };

            clap_complete::generate(clap_shell, &mut cmd, bin_name, &mut io::stdout());
        }
    }

    Ok(())
}

fn get_absolute_path(path: &str) -> Result<String> {
    let path_obj = Path::new(path);

    let expanded_tilde = if path.starts_with("~/") {
        let home = env::var("HOME")
            .or_else(|_| env::var("USERPROFILE"))
            .context("Could not determine home directory for ~ expansion")?;
        PathBuf::from(home).join(path.trim_start_matches("~/"))
    } else {
        path_obj.to_path_buf()
    };

    if expanded_tilde.exists() {
        let canonical = expanded_tilde
            .canonicalize()
            .context("Failed to canonicalize path")?;

        let mut path_str = canonical.to_string_lossy().to_string();

        if cfg!(windows) && path_str.starts_with(r"\\?\") {
            path_str = path_str.strip_prefix(r"\\?\").unwrap().to_string();
        }

        return Ok(path_str);
    }

    let absolute_path = if expanded_tilde.is_absolute() {
        expanded_tilde
    } else {
        env::current_dir()?.join(expanded_tilde)
    };

    Ok(absolute_path.to_string_lossy().to_string())
}

pub fn detect_shell() -> SupportedShell {
    let shell_env = env::var("SHELL").unwrap_or_default().to_lowercase();

    if shell_env.ends_with("zsh") {
        SupportedShell::Zsh
    } else if shell_env.ends_with("fish") {
        SupportedShell::Fish
    } else if shell_env.ends_with("nu") {
        SupportedShell::Nu
    } else {
        SupportedShell::Bash
    }
}

fn resolve_import_string(cli_arg: Option<String>) -> Result<String> {
    if let Some(arg) = cli_arg {
        return Ok(arg);
    }

    let mut stdin = io::stdin();
    if !stdin.is_terminal() {
        let mut buffer = String::new();
        stdin
            .read_to_string(&mut buffer)
            .context("Failed to read from STDIN")?;
        return Ok(buffer.trim().to_string());
    }

    env::var("PATH").context("Failed to read the PATH environment variable")
}
