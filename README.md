<div align="center">
  <h1>Paseo</h1>

[![Minimum rustc 1.94](https://img.shields.io/badge/rustc-1.94+-blue.svg)](https://rust-lang.github.io/rfcs/2495-min-rust-version.html)
[![oq3_semantics crate](https://img.shields.io/crates/v/paseo.svg)](https://crates.io/crates/paseo)

</div>

A CLI tool to elegantly manage your shell's `PATH` variable.

If you are tired of manually editing `export PATH="..."` strings in your `.bashrc` or `.zshrc`, **Paseo** is for you. It maintains your path entries cleanly in a dedicated `.pathfile` which can then be used by all your shells.

The ideal way to use **Paseo** is to let it act as the single source of truth for your `PATH`. This helps you maintain consistency if you use multiple shells in a machine (**Bash** as the default shell and **Nu** as an interactive shell for example).

**NOTE**: Paseo currently only supports Bash and Zsh. Fish and Nushell is currently under development.

## 📦 Installation

### Cargo

**Paseo** can be installed using `cargo`.

```sh
cargo install paseo
```

### Compiling from Source

You can clone this repository, build and run the program.

```sh
git clone https://github.com/madhavan-raja/paseo.git
cd ./paseo
cargo build --release
```

## 🚀 Quick Start & Setup

**Paseo** stores the managed directories in a **pathfile** (`~/.pathfile` by default, can be modified).

### Import your existing PATH

First, import your current `PATH` into **Paseo**:

```bash
paseo import
```

### Update your shell configuration

Add the following line to the end of your shell's configuration file (e.g., `.bashrc`, or `.zshrc`):

**Bash / Zsh:**

```bash
export PATH=$(paseo show --formatted)
```

### Reload your shell

```bash
source ~/.bashrc  # or restart your terminal
```

Now, whenever you add or remove directories using `paseo`, just reload your shell to apply the changes!

## 🛠️ Usage

**Paseo** provides an intuitive set of commands to manipulate your path entries.

### Show Directories

List all the directories currently managed by **Paseo**.

```bash
paseo show # aliases: list, ls
```

*Note*: Use `paseo show --formatted` to output a raw string formatted for your specific shell.

### Add a Directory

Add a new directory to **Paseo**.

```bash
paseo add /home/user/.local/bin # aliases: new, create
```

### Remove a Directory

Remove a specific directory from **Paseo**.

```bash
paseo remove /home/user/old_bin # aliases: delete, del, rm
```

### Import Paths

Import directories into **Paseo**. By default, it reads your current `$PATH` environment variable. You can also pass a raw path string, or pipe it via STDIN.

#### Import the shell's current `$PATH`

```bash
paseo import
```

#### Import a specific string
```bash
paseo import "/usr/bin:/usr/local/bin"
```

#### Import and overwrite all managed directories

```bash
paseo import --clear
```

### Restore from Backup

Before modifying the pathfile, **Paseo** automatically stores the existing directories in a backup pathfile (`~/.pathfile.backup` by default, can be modified). If you make a mistake, easily revert it:

```
paseo restore
```

Reverting swaps the contents of the main pathfile and the backup pathfile. Running the command twice will not result in any difference.

### Shell Completions

Generate tab-completions for your shell of choice (`bash`, `zsh`, `fish`).

```bash
paseo generate-completions bash > ~/.local/share/bash-completion/completions/paseo
```

## ⚙️ Global Options

You can customize **Paseo**'s behavior globally across any command:

| Option       | Short | Default              | Description                                                                       |
| :----------- | :---- | :------------------- | :-------------------------------------------------------------------------------- |
| `--shell`    | `-s`  | *Auto-detected*      | The shell format for imports and formatting output (`bash`, `zsh`, `fish`, `nu`). |
| `--pathfile` | `-p`  | `~/.pathfile`        | Location for the Paseo state file.                                                |
| `--backup`   | `-b`  | `~/.pathfile.backup` | Location for the Paseo backup file.                                               |

**Example:**

```bash
paseo --shell zsh --pathfile ~/.config/paseo/.pathfile show --formatted
```

---

## 📄 License

Copyright &copy; 2026 Madhavan Raja.

Distributed under the MIT License. See LICENSE for more information.
