//! This is a tool for parsing and querying FIF format neuroimaging data.
//!
//! # Usage
//!
//! `meginfo [file_1]`
//!
//!

use anyhow::anyhow;
use clap::Parser;
use meginfo::{config::Config, run};
use std::io;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Cli {
    #[arg(long, short)]
    file: Vec<String>,

    #[arg(long, short)]
    tag: Vec<String>,
}

const MAX_LINES_IN: u32 = 20000;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let mut files = String::new();
    let mut line_count = 0;

    while io::stdin().read_line(&mut files)? > 0 {
        line_count += 1;

        if line_count > MAX_LINES_IN {
            return Err(anyhow!(
                "refusing to read more than {MAX_LINES_IN} lines from stdin"
            ));
        }
    }

    let files: Vec<PathBuf> = files
        .lines()
        .map(|x| Path::new(x).canonicalize().unwrap())
        .filter(|x| x.is_file())
        .collect();

    if files.len() > 0 && cli.file.len() > 0 {
        return Err(anyhow!(
            "files should be supplied either from stdin or with --file (-f) argument, not both"
        ));
    }

    println!("file: {files:?}");
    let config = Config::new(files);
    run(config)?;
    Ok(())
}
