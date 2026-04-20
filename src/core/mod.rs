use std::{fs::{self, File}, io::{BufRead, BufReader}, path::PathBuf};

use sorted_insert::SortedInsert;

use crate::cli::AddArgs;

pub struct Paseo {
    pub pathfile: String
}

impl Paseo {
    pub fn new(pathfile: String) -> Self {
        println!("Initialized {}", pathfile);
        let file = fs::canonicalize(pathfile).unwrap().as_path().to_str().unwrap().to_string();
        fs::write(&file, "").unwrap();

        Paseo { pathfile: file }
    }

    pub fn list(&self) {
        println!("Listing...");
    }

    pub fn add(&self, add_args: AddArgs) {
        let directory = fs::canonicalize(add_args.directory).unwrap().as_path().to_str().unwrap().to_string();
        println!("Adding {:?}", &directory);

        // Check if path exists

        // Add to file

        let file = File::open(&self.pathfile).unwrap();
        let buf = BufReader::new(file);
        let mut lines: Vec<String> = buf.lines()
            .map(|l| l.expect("Could not parse line"))
            .collect();

        lines.sorted_insert_asc(directory);


        println!("Adding {:?}", &lines);
    }
}