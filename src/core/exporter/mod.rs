mod bash;

pub struct Exporter {
    paths: Vec<String>
}

impl Exporter {
    pub fn new(paths: Vec<String>) -> Self {
        Exporter { paths }
    }
}