use std::{collections::HashSet, fs::{self, File}, io::{BufRead, BufReader, Write}, path::PathBuf};

use tabled::settings::Style;

use crate::cli::AddArgs;

pub struct Paseo {
    pub pathfile: PathBuf
}

#[derive(tabled::Tabled)]
struct PathEntry {
    #[tabled(rename = "#")]
    index: u16,
    #[tabled(rename = "Path")]
    path: String,
    #[tabled(rename = "Is System Path?")]
    is_system_path: bool
}

impl PathEntry {
    pub fn new(index: u16, path: String, is_system_path: bool) -> Self {
        PathEntry { index, path, is_system_path }
    }
}

impl Paseo {
    pub fn new(pathfile: Option<PathBuf>) -> Self {
        let mut file;

        if let Some(file) = pathfile {
            return Paseo { pathfile: file }
        } else if let Some(config_path) = dirs::config_dir() {
            file = PathBuf::new();
            file.push(config_path);
            file.push("pathfile");

            return Paseo { pathfile: file }
        }

        panic!("Cannot find config path")
    }

    fn get_paths(&self) -> HashSet<String> {
        let file = File::open(&self.pathfile).unwrap();

        BufReader::new(file)
            .lines()
            .map(|l| l.expect("Could not parse line"))
            .collect()
    }

    fn get_paths_list(&self) -> Vec<String> {
        let mut paths_list = self.get_paths().into_iter().collect::<Vec<String>>();
        paths_list.sort();

        paths_list
    }

    fn write_paths(&self, paths: HashSet<String>) {
        let mut file = File::create(&self.pathfile).unwrap();

        let mut paths_vec = paths.into_iter().collect::<Vec<String>>();
        paths_vec.sort();

        file.write_all(paths_vec.join("\n").as_bytes()).expect("Could not write to pathfile");
    }

    fn add_path(&self, path: String) {
        let mut paths = self.get_paths();

        paths.insert(path);

        self.write_paths(paths);
    }

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
}