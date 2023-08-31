//! # meginfo is a crate for reading neuromag/MEGIN/fif files.
//!
//!

pub mod config;
pub mod enums;
pub mod graph;
pub mod parser;
pub mod query;
pub mod tag;

use std::collections::HashSet;

use anyhow;
use config::Config;
use parser::FifParser;
use query::Search;

pub fn run(config: Config) -> anyhow::Result<()> {
    if config.show_tree {
        for file in config.files {
            println!("fif tree for {file:?}: \n");
            let tags = FifParser::read_tags(file)?;
            let tree = FifParser::make_fif_tree(tags)?;
            println!("{tree}");
        }
    } else {
        let mut search = Search::new(HashSet::from_iter(config.query_codes), config.files);
        search.execute();
        println!("{search}");
    }

    Ok(())
}
