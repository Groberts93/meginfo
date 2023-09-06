//! Quickly search neuromag/MEGIN (.fif) files for specific tags.
//!
//! The .fif file format is designed for MEG, EEG and other neuroimaging data.  It is the
//! de facto internal standard used by MNE-Python, the most popular python library for
//! brain electrophysiology.  
//!
//! A single file can contain:
//! - raw time series data from the scanner
//! - metadata relating to acquisition, processing parameters, etc.
//!
//!  Such files can be several hundred megabytes in size.  This crate allows you to incrementally
//! parse the files and extract metadata without having to load the entire file into memory.
//!

pub mod config;
pub mod enums;
pub mod graph;
pub mod parser;
pub mod query;
pub mod tag;
pub mod benchtest;

use anyhow;
use config::Config;
use parser::FifParser;
use query::Search;

/// Executes the main program using the supplied Config.
///
/// Can fail if reading tags or creating the tree fails.
///
/// If show_tree is true, will print a representation of the entire fif tree for all files.
///
/// Otherwise, will search for the given tags in all supplied files.
///
pub fn run(config: Config) -> anyhow::Result<()> {
    for tag in config.describe_tags {
        println!("{tag}");
    }

    if config.show_tree {
        for file in config.files {
            println!("fif tree for {file:?}: \n");
            let tags = FifParser::read_tags(file)?;
            let tree = FifParser::make_fif_tree(tags)?;
            println!("{tree}");
        }
    } else {
        let mut search = Search::new(config.query_codes, config.files);
        search.execute();
        println!("{search}");
    }

    Ok(())
}
