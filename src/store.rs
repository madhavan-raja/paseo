use anyhow::{Context, Result};
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

pub struct PathStore {
    paths: BTreeSet<String>,
    pathfile_location: PathBuf,
}

impl PathStore {
    pub fn default_file_path() -> PathBuf {
        let home_dir = dirs::home_dir().unwrap_or_default();
        home_dir.join(".pathfile")
    }

    pub fn load(pathfile_location: PathBuf) -> Result<Self> {
        let mut paths = BTreeSet::new();

        if !pathfile_location.exists() {
            File::create_new(&pathfile_location).context(format!("Failed to create storage file: {:?}", pathfile_location))?;
        }

        let file = File::open(&pathfile_location).context(format!("Failed to open storage file: {:?}", pathfile_location))?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.context("Failed to read line")?;
            let trimmed_line = line.trim();
            
            if !trimmed_line.is_empty() {
                paths.insert(trimmed_line.to_string());
            }
        }

        Ok(Self { paths, pathfile_location })
    }

    pub fn save(&self) -> Result<()> {
        let mut file = File::create(&self.pathfile_location)
            .context(format!("Failed to create or overwrite storage file: {:?}", self.pathfile_location))?;

        for path in &self.paths {
            writeln!(file, "{}", path)?;
        }

        file.flush().context("Failed to flush data to disk")?;

        Ok(())
    }

    pub fn clear(&mut self) {
        self.paths.clear();
    }

    pub fn insert(&mut self, path: String) -> bool {
        self.paths.insert(path)
    }

    pub fn remove(&mut self, path: &str) -> bool {
        self.paths.remove(path)
    }

    pub fn get_all(&self) -> Vec<String> {
        self.paths.iter().cloned().collect()
    }
}
