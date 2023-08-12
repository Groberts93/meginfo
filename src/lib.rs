pub mod config;
mod enums;
mod graph;
mod nomparser;
mod parser;
mod tag;
mod tree;

use config::Config;
use parser::{read_tag_dict, FifParser};
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let tag_dict = read_tag_dict();
    let parser = FifParser::new(tag_dict);

    parser.parse_fif(config.file)?;

    Ok(())
}
