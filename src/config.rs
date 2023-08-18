use std::path::{Path, PathBuf};

pub struct Config {
    pub files: Vec<PathBuf>,
}

impl Config {
    pub fn new(files: Vec<PathBuf>) -> Config {
        Config { files }
    }
}
