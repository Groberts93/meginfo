use serde::Deserialize;
use std::fs::File;
use std::io::{self, BufReader, Read, Seek};

use crate::{primitive, primitive::Primitive};

#[derive(Debug)]
pub struct Tag {
    pub kind: i32, // DictionaryTags.txt
    type_: u8,     // DictionaryTypes.txt
    size: i32,     // size in bytes
    next: i32,     // offset from start of file to next tag if > 0
    pub data: u64, // data location from start of file
}

impl Tag {
    pub fn new(ints: &[i32; 4]) -> Self {
        Tag {
            kind: ints[0],
            type_: ints[1] as u8,
            size: ints[2],
            next: ints[3],
            data: 0,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TagDef {
    pub code: i32,
    name: String,
    dtype: String,
    unit: String,
    description: String,
}

impl Default for TagDef {
    fn default() -> Self {
        TagDef {
            code: 0,
            name: "unknown".to_string(),
            dtype: "unknown".to_string(),
            unit: "unknown".to_string(),
            description: "Unrecognized tag".to_string(),
        }
    }
}

impl Tag {
    pub fn read_data(&self, fh: &File) -> Primitive {
        let mut reader = BufReader::new(fh);
        let mut buffer = vec![0; self.size as usize];

        reader.read_exact(&mut buffer).unwrap();
        let prim = primitive::parse_primitive(&buffer, self.type_);
        prim
    }
}
