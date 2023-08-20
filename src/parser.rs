use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Seek};
use std::path::PathBuf;
use std::vec;

use crate::enums::{BlockTagKind, DataTagKind};
use crate::graph::Tree;

use crate::tag::{tag_header, Block, FiffNode, Tag, TagDef};
use csv::ReaderBuilder;

// contains main file reading and parsing loop

pub struct FifParser {
    query_tags: Vec<i32>,
}

impl FifParser {
    pub fn new(query_tags: Vec<i32>) -> Self {
        FifParser { query_tags }
    }

    pub fn parse_fif(&self, file: PathBuf) -> Result<Tree<FiffNode>> {
        // open the fif file, wrap in bufreader
        let fh = File::open(&file).with_context(|| format!("No file found at {:?}", &file))?;

        let mut tree = self.collect_tags(fh)?;

        Ok(tree)
    }

    fn collect_tags(&self, fh: File) -> Result<Tree<FiffNode>> {
        let file_length = fh.metadata().unwrap().len();

        const BUFFER_SIZE: usize = 8192;

        let mut reader = io::BufReader::with_capacity(BUFFER_SIZE, fh);
        let mut header_buf = [0u8; 16];
        let mut tags = vec![];

        let mut position = 0u64;
        let mut search_results = vec![];

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

            match &tag {
                Tag::Data { kind, .. } => {
                    for code in &self.query_tags {
                        if DataTagKind::from_code(*code) == *kind {
                            search_results.push(tag.clone());
                        }
                    }
                }
                _ => {}
            }
            tags.push(tag);
        }

        println!("{:?}", search_results);

        let mut tree = Tree::new();
        let mut stack = vec![];
        let mut curr = tree.root;

        for tag in tags {
            match &tag {
                Tag::Block { kind, .. } => match kind {
                    BlockTagKind::BlockStart => {
                        stack.push(curr);
                        let child = tree.add_child(FiffNode::from_tag(tag));
                        tree.move_to(child);
                        curr = child;
                    }
                    BlockTagKind::BlockEnd => {
                        tree.move_to(stack.pop().unwrap());
                    }
                    _ => {
                        println!("ignored tag: {:?}", &tag)
                    }
                },
                Tag::Data { .. } => {
                    tree.add_child(FiffNode::from_tag(tag));
                }
            }
        }

        let cur_pos = reader
            .seek(io::SeekFrom::Current(0))
            .expect("should be able to seek to current position");

        println!(
            "Finished reading, cursor at {} bytes (tracked {}), file is {} bytes long",
            cur_pos, position, file_length
        );

        Ok(tree)
    }
}
