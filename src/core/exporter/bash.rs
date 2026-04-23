use crate::core::exporter::Exporter;

impl Exporter {
    pub fn export_bash(&self) -> String {
        self.paths.join(":")
    }
}