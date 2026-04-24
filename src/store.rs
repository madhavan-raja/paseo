use anyhow::{Context, Result};
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

pub struct PathStore {
    paths: BTreeSet<String>,
    file_path: PathBuf,
}

impl PathStore {
    fn default_file_path() -> Result<PathBuf> {
        let home_dir = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE")) // Fallback for Windows
            .context("Could not determine the user's home directory")?;
            
        Ok(PathBuf::from(home_dir).join("pathfile"))
    }

    pub fn load(pathfile_location: Option<PathBuf>) -> Result<Self> {
        let file_path = pathfile_location.unwrap_or(Self::default_file_path()?);
        let mut paths = BTreeSet::new();

        if file_path.exists() {
            let file = File::open(&file_path)
                .with_context(|| format!("Failed to open storage file: {:?}", file_path))?;
            let reader = BufReader::new(file);

            for line in reader.lines() {
                let line = line?;
                let trimmed = line.trim();
                
                if !trimmed.is_empty() {
                    paths.insert(trimmed.to_string());
                }
            }
        }

        Ok(Self { paths, file_path })
    }

    /// Saves the current BTreeSet to disk, writing each path on a new line.
    /// Because we iterate over a BTreeSet, the output is guaranteed to be alphabetical.
    pub fn save(&self) -> Result<()> {
        let mut file = File::create(&self.file_path)
            .with_context(|| format!("Failed to create or overwrite storage file: {:?}", self.file_path))?;

        for path in &self.paths {
            writeln!(file, "{}", path)?;
        }

        file.flush().context("Failed to flush data to disk")?;

        Ok(())
    }

    /// Inserts a new path. Returns `true` if the path was newly added, 
    /// or `false` if it was already present in the store.
    pub fn insert(&mut self, path: String) -> bool {
        self.paths.insert(path)
    }

    /// Removes a path. Returns `true` if the path existed and was removed,
    /// or `false` if it wasn't in the store to begin with.
    pub fn remove(&mut self, path: &str) -> bool {
        self.paths.remove(path)
    }

    /// Returns an alphabetically ordered list of all paths.
    pub fn get_all(&self) -> Vec<String> {
        self.paths.iter().cloned().collect()
    }
}
