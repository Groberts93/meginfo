//! This is a tool for parsing and querying FIF format neuroimaging data.
//!
//! # Usage
//!
//! Look at one file with:
//!
//! `meginfo -f file.fif`
//!
//! A file list can be read from stdin:
//!
//! `find data | meginfo`
//!

use anyhow::anyhow;
use atty::Stream;
use clap::Parser;
use meginfo::{config::Config, run};
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

use env_logger;
use log::{LevelFilter, info, warn};

#[derive(Parser)]
#[command(version, about, long_about = None)] // Read from `Cargo.toml`
struct Cli {
    #[arg(long, short)]
    files: Vec<String>,

    #[arg(long, short)]
    tags: Vec<String>,

    #[arg(long, short)]
    show_tree: bool,

    #[arg(long, short)]
    log: Option<LevelFilter>,
}

const MAX_LINES_IN: u32 = 20000;

fn strings_to_filepaths(input: Vec<String>) -> Vec<PathBuf> {
    input
        .iter()
        .map(|x| Path::new(x).canonicalize().unwrap())
        .filter(|x| x.is_file())
        .collect()
}

fn main() -> anyhow::Result<()> {


    let cli = Cli::parse();

    let log_level = cli.log.unwrap_or(LevelFilter::Warn);
    env_logger::Builder::new().filter_level(log_level).init();

    let mut files: Vec<String> = vec![];
    let mut line_count = 0;

    // read files, either from stdin or parsing arguments
    let files = if atty::isnt(Stream::Stdin) {
        if !cli.files.is_empty() {
            return Err(anyhow!(
                "files should be supplied either from stdin or with --file (-f) argument, not both"
            ));
        }

        let stdin = io::stdin();

        for line in stdin.lock().lines() {
            files.push(line?);
            line_count += 1;

            if line_count > MAX_LINES_IN {
                return Err(anyhow!(
                    "refusing to read more than {MAX_LINES_IN} lines from stdin"
                ));
            }
        }

        files
    } else {
        cli.files
    };

    if files.is_empty() {
        return Err(anyhow!(
            "Need at least one file to read.  Use --file (-f) or pass file list to stdin."
        ));
    }

    let files = strings_to_filepaths(files);
    let config = Config::new(files, cli.show_tree, cli.tags, true);
    run(config)?;
    Ok(())
}
