use meginfo::run;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // read tags from tsv into a dictionary

    run()?;

    Ok(())
}
