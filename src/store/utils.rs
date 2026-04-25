use crate::store::PathStore;
use anyhow::{Context, Result};
use std::collections::BTreeSet;

use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

impl PathStore {
    pub(super) fn read_directories(file_location: &PathBuf) -> Result<BTreeSet<String>> {
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

    pub(super) fn write_directories(file_location: &PathBuf, directories: &BTreeSet<String>) -> Result<()> {
        let mut file = File::create(file_location)
            .context(format!("Failed to create or overwrite file: {:?}", file_location))?;

        for path in directories {
            writeln!(file, "{}", path)?;
        }

        file.flush().context("Failed to flush data to disk")?;

        Ok(())
    }
}