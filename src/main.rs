//! This is a binary for parsing and querying FIF format neuroimaging data.
//!
//! # Usage
//!
//! `meginfo [file_1]`
//!
//!

use meginfo::{config::Config, run};
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // read tags from tsv into a dictionary

    let mut args = env::args();
    args.next().unwrap();
    let mut args: Vec<String> = args.collect();

    let file = args.pop().ok_or("No file supplied")?;
    let config = Config::from_file(file);
    run(config)?;
    Ok(())
}
