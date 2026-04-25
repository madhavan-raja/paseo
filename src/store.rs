use anyhow::{Context, Result};
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

pub struct PathStore {
    paths_unchanged: BTreeSet<String>,
    paths: BTreeSet<String>,
    pathfile_location: PathBuf,
    pathfile_backup_location: PathBuf
}

impl PathStore {
    pub fn default_pathfilefile_path() -> PathBuf {
        let home_dir = dirs::home_dir().unwrap_or_default();
        home_dir.join(".pathfile")
    }

    pub fn default_pathfile_backup_path() -> PathBuf {
        let home_dir = dirs::home_dir().unwrap_or_default();
        home_dir.join(".pathfile.backup")
    }

    pub fn load(pathfile_location: PathBuf, pathfile_backup_location: PathBuf) -> Result<Self> {
        if !pathfile_location.exists() {
            File::create_new(&pathfile_location).context(format!("Failed to create pathfile: {:?}", pathfile_location))?;
        }

        if !pathfile_location.exists() {
            File::create_new(&pathfile_location).context(format!("Failed to create pathfile backup: {:?}", pathfile_backup_location))?;
        }

        let paths = Self::read_directories(&pathfile_location)?;

        Ok(Self { paths_unchanged: paths.clone(), paths, pathfile_location, pathfile_backup_location })
    }

    fn read_directories(file_location: &PathBuf) -> Result<BTreeSet<String>> {
        let file = File::open(file_location).context(format!("Failed to open file: {:?}", file_location))?;

        let mut directories = BTreeSet::new();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.context("Failed to read line")?;
            let trimmed_line = line.trim();
            
            if !trimmed_line.is_empty() {
                directories.insert(trimmed_line.to_string());
            }
        }

        Ok(directories)
    }

    pub fn save(&self) -> Result<()> {
        Self::write_directories(&self.pathfile_backup_location, &self.paths_unchanged)?;
        Self::write_directories(&self.pathfile_location, &self.paths)?;

        Ok(())
    }

    pub fn restore(&self) -> Result<()> {
        let paths_backup = Self::read_directories(&self.pathfile_backup_location)?;

        Self::write_directories(&self.pathfile_location, &paths_backup)?;
        Self::write_directories(&self.pathfile_backup_location, &self.paths)?;

        Ok(())
    }

    fn write_directories(file_location: &PathBuf, directories: &BTreeSet<String>) -> Result<()> {
        let mut file = File::create(file_location)
            .context(format!("Failed to create or overwrite file: {:?}", file_location))?;

        for path in directories {
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
