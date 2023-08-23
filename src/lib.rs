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
    let parser = FifParser::new(config.query_codes);

    for file in config.files {
        let tags = parser.read_tags(file)?;
        if config.show_tree {
            let tree = parser.make_fif_tree(tags)?;
            println!("{}", tree);
        }
    }

    Ok(())
}
