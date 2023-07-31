use csv::ReaderBuilder;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read};

#[derive(Debug)]
struct Tag {
    kind: i32,  // DictionaryTags.txt
    type_: i32, // DictionaryTypes.txt
    size: i32,  // size in bytes
    next: i32,  // offset from start of file to next tag if > 0
}

// struct Block {

// }

impl Tag {
    fn new(ints: &[i32; 4]) -> Self {
        Tag {
            kind: ints[0],
            type_: ints[1],
            size: ints[2],
            next: ints[3],
        }
    }
}

#[derive(Debug, Deserialize, Default)]
struct TagDef {
    name: String,
    code: i32,
    dtype: String,
    unit: String,
    description: String,
}

fn main() -> Result<(), Box<dyn Error>> {

    // read tags from tsv into a dictionary
    let mut reader = ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path("fiff/tags.tsv")
        .expect("file should be found in fiff/tags.tsv");

    let mut tag_dict: HashMap<i32, TagDef> = HashMap::new();

    for result in reader.deserialize() {
        let record: TagDef = result?;
        tag_dict.insert(record.code, record);
    }

    // open the fif file, wrap in bufreader
    let fh = File::open("data/file_2.fif").unwrap();
    let mut reader = io::BufReader::new(fh);
    let mut buf = [0u8; 16];

    // read tags sequentially until we find the end (no op) tag
    while let Ok(()) = reader.read_exact(&mut buf) {
        let (tag, size) = parse_tag_from_bytes(&buf);
        println!(
            "{:?}",
            tag_dict.get(&tag.kind).unwrap_or(&TagDef {
                code: tag.kind,
                name: "unknown".to_string(),
                description: "Unrecognized tag".to_string(),
                ..Default::default()
            })
        );
        reader.seek_relative(size)?;
    }

    println!("Finished reading");

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
