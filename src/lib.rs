//! # meginfo is a crate for reading neuromag/MEGIN/fif files.
//!
//!

pub mod config;
pub mod enums;
pub mod graph;
pub mod parser;
pub mod query;
pub mod tag;

use anyhow;
use config::Config;
use parser::FifParser;

pub fn run(config: Config) -> anyhow::Result<()> {
    for file in config.files {
        let tags = FifParser::read_tags(file)?;
        if config.show_tree {
            let tree = FifParser::make_fif_tree(tags)?;
            println!("{}", tree);
        }
    }

    Ok(())
}
