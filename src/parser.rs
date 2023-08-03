use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Seek};

use crate::tag::{Tag, TagDef};
use csv::ReaderBuilder;

pub struct FifParser {
    tag_dict: HashMap<i32, TagDef>,
}

impl FifParser {
    pub fn new(tag_dict: HashMap<i32, TagDef>) -> Self {
        FifParser { tag_dict }
    }

    pub fn parse_fif(&self, file: String) -> io::Result<()> {
        // open the fif file, wrap in bufreader
        let fh = File::open(&file).unwrap();

        let mut tags = self.collect_tags(fh);

        
        let fh = File::open(&file).unwrap();
        
        for _ in 0..3 {
            
            let tag = tags.pop().unwrap();
            println!("{:?}", tag.read_data(&fh));
        }
        Ok(())
    }

    fn collect_tags(&self, fh: File) -> Vec<Tag> {
        let file_length = fh.metadata().unwrap().len();
        let default_tag_def = TagDef::default();

        let mut reader = io::BufReader::new(fh);
        let mut buf = [0u8; 16];

        let mut tags = vec![];

        // read tags sequentially until we find the end (no op) tag
        while let Ok(()) = reader.read_exact(&mut buf) {
            let (mut tag, size) = parse_tag_from_bytes(&buf);

            tag.data = reader
                .seek(io::SeekFrom::Current(0))
                .expect("should be able to seek to current position");

            println!(
                "{:?}, {:?}",
                &self.tag_dict.get(&tag.kind).unwrap_or(&default_tag_def),
                tag
            );
            tags.push(tag);
            reader.seek_relative(size).unwrap();
        }

        let cur_pos = reader
            .seek(io::SeekFrom::Current(0))
            .expect("should be able to seek to current position");
        println!(
            "Finished reading, cursor at {} bytes, file is {} bytes long",
            cur_pos, file_length
        );

        tags
    }
}

fn parse_tag_from_bytes(bytes: &[u8; 16]) -> (Tag, i64) {
    let mut ints = [0i32; 4];

    for (ii, chunk) in bytes.chunks_exact(4).enumerate() {
        let value = i32::from_be_bytes(chunk.try_into().unwrap());
        ints[ii] = value;
    }
    (Tag::new(&ints), ints[2].try_into().unwrap())
}

pub fn read_tag_dict() -> HashMap<i32, TagDef> {
    let mut reader = ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path("fiff/tags.tsv")
        .expect("file should be found in fiff/tags.tsv");

    let mut tag_dict: HashMap<i32, TagDef> = HashMap::new();

    for result in reader.deserialize() {
        let record: TagDef = result.expect("static tsv should have been readable");
        tag_dict.insert(record.code, record);
    }

    tag_dict
}
