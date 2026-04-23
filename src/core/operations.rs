use std::fs;

use crate::cli::{AddArgs, ExportArgs, ImportArgs};
use crate::core::paseo::Paseo;
use crate::core::path_entry::PathEntry;

use tabled::settings::Style;

impl Paseo {
    pub fn list(&self) {
        let paths = self.get_paths_list();

        let mut index = 0;
        let path_entries = paths.into_iter().map(|path| { index += 1; PathEntry::new(index, path, false) }).collect::<Vec<PathEntry>>();

        let mut table = tabled::Table::new(path_entries);
        table.with(Style::rounded());

        println!("{}", table);
    }

    pub fn add(&self, add_args: AddArgs) {
        let directory = fs::canonicalize(add_args.directory).unwrap().as_path().to_str().unwrap().to_string();
        self.add_path(directory);
    }

    pub fn export(&self, export_args: ExportArgs) {
        let path_value = self.get_exported_path(export_args.shell);
        println!("{}", path_value);
    }

    pub fn import(&self, import_args: ImportArgs) {
        let paths = self.get_imported_paths(import_args.shell, import_args.paths);
        self.write_paths(paths);
    }
}