//! Tag
//! 
//! This is an abstraction to contain tags read from a fif file.
//! Each tag comprises a header and a data block.
//! 
//! The header has a fixed size of 16 bytes, while the data block is variable-size.
//! Each header is decoded as four i32 values:
//! 
//! code: an enum which defines the "kind" of tag, see fiff/tags.tsv
//! dtype: an enum which defines the format of the data block, see fiff/primitives.tsv
//! size: the size in bytes of the ensuing data block
//! next: supposedly a file pointer to the next block, but typically set to 0 and ignored
//! 
//! Contains code to parse these from u8 slices using nomparser.
//! 

use nom::multi;
use nom::number::complete::{be_f32, be_i32};
use nom::{sequence, IResult};
use std::fmt::Display;

use crate::enums::{Block, Kind};
use serde::Deserialize;

#[derive(Debug, PartialEq, Default)]
pub struct Tag {
    pub kind: Kind,
    data: Data,
}

impl Tag {
    pub fn from_header_slice(header: TagHeader, slice: Vec<u8>) -> Self {
        Tag {
            kind: Kind::from_code(header.code),
            data: Data::from_slice(slice, header.dtype),
        }
    }

    pub fn from_header_file_position(header: TagHeader, start: u64, size: u64) -> Self {
        Tag {
            kind: Kind::from_code(header.code),
            data: Data::InFile {
                start: start,
                size: size,
            },
        }
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
// the tag header struct, corresponds exactly to the 16 byte headers in the file
#[derive(Debug)]
pub struct TagHeader {
    pub code: i32,
    pub dtype: i32,
    pub size: i32,
    pub next: i32,
}

// data for a tag, either owns the actual data (for small data) or data position in the file
// (for large data that requires deferred reading)
#[derive(Debug, PartialEq, Clone, Default)]
pub enum Data {
    #[default]
    Void,
    Slice(Vec<u8>),
    InFile {
        start: u64,
        size: u64,
    },
    Int32(Vec<i32>),
    Float(Vec<f32>),
    JulianDate(Vec<i32>),
    String(String),
    ChInfoStruct(Vec<u8>),
    IdStruct(Vec<u8>),
    DigPointStruct(Vec<u8>),
    CoordTransStruct(Vec<u8>),
}

impl Data {
    pub fn from_slice(slice: Vec<u8>, dtype: i32) -> Self {
        match dtype {
            0 => Data::Void,
            3 => Data::Int32(i32_many(&slice).unwrap().1),
            4 => Data::Float(f32_many(&slice).unwrap().1),
            6 => Data::JulianDate(i32_many(&slice).unwrap().1),
            10 => Data::String(string(slice)),
            30 => Data::ChInfoStruct(slice),
            31 => Data::IdStruct(slice),
            33 => Data::DigPointStruct(slice),
            35 => Data::CoordTransStruct(slice),
            _ => Data::Slice(slice),
        }
    }
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
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

#[derive(Debug, PartialEq)]
pub enum FiffNode {
    Block(Block),
    Tag(Tag),
}

impl Default for FiffNode {
    fn default() -> Self {
        FiffNode::Block(Block::Root)
    }
}

impl Display for FiffNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn tag_header(input: &[u8]) -> IResult<&[u8], (u64, TagHeader)> {
    let (input, (code, dtype, size, next)) =
        sequence::tuple((be_i32, be_i32, be_i32, be_i32))(input)?;

    Ok((
        input,
        (
            size as u64,
            TagHeader {
                code,
                dtype,
                size,
                next,
            },
        ),
    ))
}

pub fn i32_many(input: &[u8]) -> IResult<&[u8], Vec<i32>> {
    multi::many1(be_i32)(input)
}

pub fn f32_many(input: &[u8]) -> IResult<&[u8], Vec<f32>> {
    multi::many1(be_f32)(input)
}

pub fn string(input: Vec<u8>) -> String {
    String::from_utf8(input).unwrap()
}
