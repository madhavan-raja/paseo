mod bash;

pub struct Importer {
    paths: String
}

impl Importer {
    pub fn new(paths: String) -> Self {
        Importer { paths }
    }
}