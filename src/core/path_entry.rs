#[derive(tabled::Tabled)]
pub struct PathEntry {
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