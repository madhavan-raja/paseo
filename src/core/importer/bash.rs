use std::collections::HashSet;

use crate::core::importer::Importer;

impl Importer {
    pub fn export_bash(&self) -> HashSet<String> {
        self.paths.split(":").map(String::from).collect()
    }
}