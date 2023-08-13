//! # meginfo is a crate for reading neuromag/MEGIN/fif files.
//! 
//! 

pub mod config;
pub mod enums;
pub mod graph;
pub mod parser;
pub mod tag;

use config::Config;
use parser::{read_tag_dict, FifParser};
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let tag_dict = read_tag_dict();
    let parser = FifParser::new(tag_dict);

    parser.parse_fif(config.file)?;

    Ok(())
}
