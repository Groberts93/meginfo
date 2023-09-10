//! Parse a .fif file into tree structure or vector of tags

use anyhow::{Context, Result};
use log::{info, warn};
use std::fs::File;
use std::io::{self, Read, Seek};
use std::path::PathBuf;
use std::vec;

use crate::enums::BlockTagKind;
use crate::graph::Tree;

use crate::tag::{tag_header, FiffNode, Tag};

// contains main file reading and parsing loop

pub struct FifParser;

impl FifParser {
    pub fn parse(file: PathBuf) -> Result<Tree<FiffNode>> {
        let tags = Self::read_tags(file)?;
        let tree = Self::make_fif_tree(tags)?;
        Ok(tree)
    }

    pub fn read_tags(file: PathBuf) -> Result<Vec<Tag>> {
        let fh = File::open(&file).with_context(|| format!("No file found at {:?}", &file))?;

        let file_length = fh.metadata().unwrap().len();

        const BUFFER_SIZE: usize = 8192;
        const MAX_PARSE_SIZE: u64 = 512;

        let mut reader = io::BufReader::with_capacity(BUFFER_SIZE, fh);
        let mut header_buf = [0u8; 16];
        let mut tags: Vec<Tag> = vec![];

        let mut position = 0u64;

        while let Ok(()) = reader.read_exact(&mut header_buf) {
            let (_, (size, tag_header)) = tag_header(&header_buf).unwrap();
            position += 16;

            let tag = if size > MAX_PARSE_SIZE {
                reader.seek_relative(size as i64).unwrap();
                Tag::from_header_file_position(tag_header, position, size)
            } else {
                let mut data_buf = vec![0; size as usize];
                reader.read_exact(&mut data_buf).unwrap();
                Tag::from_header_slice(tag_header, data_buf)
            };

            position += size;

            match tag {
                Ok(tag) => tags.push(tag),
                Err(e) => warn!("{e}"),
            }
        }

        let cur_pos = reader
            .seek(io::SeekFrom::Current(0))
            .expect("should be able to seek to current position");

        info!(
            "Finished reading, cursor at {} bytes (tracked {}), file is {} bytes long",
            cur_pos, position, file_length
        );

        Ok(tags)
    }

    pub fn make_fif_tree(tags: Vec<Tag>) -> Result<Tree<FiffNode>> {
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
                        info!("ignored tag: {:?}", &tag)
                    }
                },
                Tag::Data { .. } => {
                    tree.add_child(FiffNode::from_tag(tag));
                }
            }
        }

        Ok(tree)
    }
}
