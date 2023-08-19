use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Config {
    pub files: Vec<PathBuf>,
    pub show_tree: bool,
}

impl Config {
    pub fn new(files: Vec<PathBuf>, show_tree: bool) -> Config {
        Config { files, show_tree }
    }
}
