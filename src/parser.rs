use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Seek};
use std::vec;

use crate::nomparser::tag_header;
use crate::tag::{Tag, TagDef};
use csv::ReaderBuilder;

pub struct FifParser {
    _tag_dict: HashMap<i32, TagDef>,
}

impl FifParser {
    pub fn new(_tag_dict: HashMap<i32, TagDef>) -> Self {
        FifParser { _tag_dict }
    }

    pub fn parse_fif(&self, file: String) -> io::Result<()> {
        // open the fif file, wrap in bufreader
        let fh = File::open(&file).unwrap();

        let mut _tags = self.collect_tags(fh);

        let _fh = File::open(&file).unwrap();

        Ok(())
    }

    fn collect_tags(&self, fh: File) -> Vec<Tag> {
        let file_length = fh.metadata().unwrap().len();

        const BUFFER_SIZE: usize = 8192;

        let mut reader = io::BufReader::with_capacity(BUFFER_SIZE, fh);
        let mut header_buf = [0u8; 16];
        let mut tags = vec![];

        let mut position = 0u64;

        while let Ok(()) = reader.read_exact(&mut header_buf) {
            let (_, (size, tag_header)) = tag_header(&header_buf).unwrap();
            position += 16;

            let tag = if size > 30 {
                reader.seek_relative(size as i64).unwrap();
                Tag::from_header_file_position(tag_header, position, size)
            } else {
                let mut data_buf = vec![0; size as usize];
                reader.read_exact(&mut data_buf).unwrap();
                Tag::from_header_slice(tag_header, data_buf)
            };

            position += size;

            tags.push(tag);
        }

        for tag in &tags {
            println!("{:?}", tag);
        }

        let cur_pos = reader
            .seek(io::SeekFrom::Current(0))
            .expect("should be able to seek to current position");

        println!(
            "Finished reading, cursor at {} bytes (tracked {}), file is {} bytes long",
            cur_pos, position, file_length
        );

        tags
    }
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
