//! # meginfo is a crate for reading neuromag/MEGIN/fif files.
//!
//!

pub mod config;
pub mod enums;
pub mod graph;
pub mod parser;
pub mod tag;

use anyhow;
use config::Config;
use parser::{read_tag_dict, FifParser};

pub fn run(config: Config) -> anyhow::Result<()> {
    let tag_dict = read_tag_dict();
    let parser = FifParser::new(tag_dict);

    let file = config.files[0].clone();

    parser.parse_fif(file)?;

    Ok(())
}
