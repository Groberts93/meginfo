mod enums;
mod graph;
mod nomparser;
mod parser;
mod tag;
mod tree;

use parser::{read_tag_dict, FifParser};
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let tag_dict = read_tag_dict();
    let parser = FifParser::new(tag_dict);

    parser.parse_fif("data/file_0.fif".to_string())?;

    Ok(())
}
