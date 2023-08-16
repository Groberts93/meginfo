use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Seek};
use std::vec;

use crate::enums::{BlockTagKind, DataTagKind};
use crate::graph::Tree;

use crate::tag::{tag_header, Block, BlockTag, DataTag, FiffNode, Tag, TagDef};
use csv::ReaderBuilder;

// contains main file reading and parsing loop

// is this a comment?
pub struct FifParser {
    tag_dict: HashMap<i32, TagDef>,
}

impl FifParser {
    pub fn new(tag_dict: HashMap<i32, TagDef>) -> Self {
        FifParser { tag_dict }
    }

    pub fn parse_fif(&self, file: String) -> Result<()> {
        // open the fif file, wrap in bufreader
        let fh = File::open(&file).with_context(|| format!("No file found at {}", &file))?;

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

        let mut codes = HashMap::new();

        while let Ok(()) = reader.read_exact(&mut header_buf) {
            let (_, (size, tag_header)) = tag_header(&header_buf).unwrap();
            position += 16;

            codes
                .entry(tag_header.code)
                .and_modify(|x| *x += 1)
                .or_insert(1);

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

        let mut tree = Tree::new();
        let mut stack = vec![];
        let mut curr = tree.root;

        for tag in tags {
            match tag {
                Tag::BlockTag(block) => match block.kind {
                    BlockTagKind::BlockStart => {
                        stack.push(curr);
                        let child = tree.add_child(FiffNode::from_block_tag(block));
                        tree.move_to(child);
                        curr = child;
                    }
                    BlockTagKind::BlockEnd => {
                        tree.move_to(stack.pop().unwrap());
                    }
                    BlockTagKind::BlockId => {
                        // TODO: put this in the block structure
                    }
                },
                Tag::DataTag(data) => {
                    tree.add_child(FiffNode::from_data_tag(data));
                }
            }
        }

        println!("{}", tree);

        let cur_pos = reader
            .seek(io::SeekFrom::Current(0))
            .expect("should be able to seek to current position");

        println!(
            "Finished reading, cursor at {} bytes (tracked {}), file is {} bytes long",
            cur_pos, position, file_length
        );

        vec![]
    }
}

pub fn read_tag_dict() -> HashMap<i32, TagDef> {
    //! how bout this?
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
