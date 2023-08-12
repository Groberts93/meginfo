pub struct Config {
    pub file: String,
}

impl Config {
    pub fn from_file(file: String) -> Config {
        Config { file }
    }
}
