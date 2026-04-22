use std::{collections::HashSet, fs::{self, File}, io::{BufRead, BufReader, Write}, path::PathBuf};

use crate::cli::AddArgs;

pub struct Paseo {
    pub pathfile: PathBuf
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
        let lines = self.get_paths_list();

        println!("{:?}", lines);
    }

    pub fn add(&self, add_args: AddArgs) {
        let directory = fs::canonicalize(add_args.directory).unwrap().as_path().to_str().unwrap().to_string();

        self.add_path(directory);
    }
}