use csv::ReaderBuilder;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read, Seek};

mod tag;

use tag::{Tag, TagDef};

fn main() -> Result<(), Box<dyn Error>> {
    // read tags from tsv into a dictionary
    let mut reader = ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path("fiff/tags.tsv")
        .expect("file should be found in fiff/tags.tsv");

    let mut tag_dict: HashMap<i32, tag::TagDef> = HashMap::new();

    for result in reader.deserialize() {
        let record: TagDef = result?;
        tag_dict.insert(record.code, record);
    }

    // open the fif file, wrap in bufreader
    let fh = File::open("data/file_2.fif").unwrap();
    let file_length = fh.metadata().unwrap().len();
    let mut reader = io::BufReader::new(fh);
    let mut buf = [0u8; 16];

    // read tags sequentially until we find the end (no op) tag
    while let Ok(()) = reader.read_exact(&mut buf) {
        let (tag, size) = parse_tag_from_bytes(&buf);

        let default_tag_def = TagDef::default();
        println!("{:?}", tag_dict.get(&tag.kind).unwrap_or(&default_tag_def));
        reader.seek_relative(size)?;
    }

    let cur_pos = reader
        .seek(io::SeekFrom::Current(0))
        .expect("should be able to seek to current position");
    println!(
        "Finished reading, cursor at {} bytes, file is {} bytes long",
        cur_pos, file_length
    );

    Ok(())
}

fn parse_tag_from_bytes(bytes: &[u8; 16]) -> (Tag, i64) {
    let mut ints = [0i32; 4];

    for (ii, chunk) in bytes.chunks_exact(4).enumerate() {
        let value = i32::from_be_bytes(chunk.try_into().unwrap());
        ints[ii] = value;
    }
    (Tag::new(&ints), ints[2].try_into().unwrap())
}
