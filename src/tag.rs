//! Model for the tag element of a .fif file.
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

use csv::ReaderBuilder;
use nom::number::complete::{be_f32, be_i32};
use nom::{multi, AsBytes};
use nom::{sequence, IResult};
use std::collections::HashMap;
use std::fmt::Display;

use anyhow::Result;

use crate::enums::{BlockKind, BlockTagKind, DataTagKind};
use serde::Deserialize;

#[derive(Debug, PartialEq)]
pub enum FiffNode {
    Tag { kind: DataTagKind, data: Data },
    Block { kind: BlockKind },
}

impl FiffNode {
    pub fn from_tag(tag: Tag) -> Self {
        match tag {
            Tag::Block { kind, data } => match kind {
                BlockTagKind::BlockStart => {
                    if let Data::Int32(data) = data {
                        let kind = BlockKind::from_code(data[0]);
                        FiffNode::Block { kind }
                    } else {
                        panic!("block start has non-int32 dtype")
                    }
                }
                _ => panic!("tried to created block using non-start code"),
            },

            Tag::Data { kind, data } => FiffNode::Tag {
                kind: kind,
                data: data,
            },
        }
    }
}

impl Default for FiffNode {
    fn default() -> Self {
        FiffNode::Block {
            kind: BlockKind::Root,
        }
    }
}

impl Display for FiffNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct Block {
    pub kind: BlockKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Tag {
    Data { kind: DataTagKind, data: Data },
    Block { kind: BlockTagKind, data: Data },
}

impl Default for Tag {
    fn default() -> Self {
        Tag::Data {
            kind: DataTagKind::Nop,
            data: Data::Void,
        }
    }
}

impl Tag {
    pub fn from_header_slice(header: Header, slice: Vec<u8>) -> Result<Self> {
        // see if it's a block code first
        if let Ok(kind) = BlockTagKind::from_code(header.code) {
            return Ok(Tag::Block {
                kind,
                data: Data::from_slice(slice, header.dtype), // TODO: implement this properly
            });
        // otherwise we assume it's a normal tag
        } else {
            return Ok(Tag::Data {
                kind: DataTagKind::from_code(header.code)?,
                data: Data::from_slice(slice, header.dtype),
            });
        }
    }

    pub fn from_header_file_position(header: Header, start: u64, size: u64) -> Result<Self> {
        Ok(Tag::Data {
            kind: DataTagKind::from_code(header.code)?,
            data: Data::InFile {
                start: start,
                size: size,
            },
        })
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
// the tag header struct, corresponds exactly to the 16 byte headers in the file
#[derive(Debug)]
pub struct Header {
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
    IdStruct(IdStruct),
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
            31 => Data::IdStruct(idstruct(&slice).unwrap().1),
            33 => Data::DigPointStruct(slice),
            35 => Data::CoordTransStruct(slice),
            _ => Data::Slice(slice),
        }
    }
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let disp = match self {
            Data::Float(x) => display_vec(x),
            Data::Int32(x) => display_vec(x),
            Data::String(x) => format!("{x}"),
            x => {
                format!("{x:?}")
            }
        };

        write!(f, "{}", disp)
    }
}

fn display_vec<T: Display>(input: &[T]) -> String {
    match input.len() {
        0 => String::from("None"),
        1 => format!("{}", input[0]),
        _ => {
            let strings: Vec<String> = input.iter().map(|x| x.to_string()).collect();
            strings.join(" ")
        }
    }
}

pub struct LabelledData(DataTagKind, Data);

impl LabelledData {
    pub fn new(kind: DataTagKind, data: Data) -> Self {
        LabelledData(kind, data)
    }
}

impl Display for LabelledData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            LabelledData(DataTagKind::MeasDate, Data::Int32(data)) => {
                format!("{}", decode_unix_date(data))
            }
            LabelledData(_, data) => format!("{data}"),
        };

        write!(f, "{}", out)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct IdStruct {
    version: i32,
    machid: (i32, i32),
    secs: i32,
    usecs: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TagDef {
    pub code: i32,
    pub name: String,
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

impl Display for TagDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = format!(
            "{} ({}): {} ({})",
            self.name, self.dtype, self.description, self.unit
        );
        write!(f, "{}", out)
    }
}

pub fn read_tag_dict() -> HashMap<String, TagDef> {
    let file = include_bytes!("../fiff/tags.tsv");
    let mut reader = ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(file.as_bytes());

    let mut string_to_tag: HashMap<String, TagDef> = HashMap::new();

    for result in reader.deserialize() {
        let record: TagDef = result.expect("static tsv should have been readable");

        if let Ok(_) = DataTagKind::from_code(record.code) {
            string_to_tag.insert(record.name.clone(), record);
        }
    }

    string_to_tag
}

pub fn tag_header(input: &[u8]) -> IResult<&[u8], (u64, Header)> {
    let (input, (code, dtype, size, next)) =
        sequence::tuple((be_i32, be_i32, be_i32, be_i32))(input)?;

    Ok((
        input,
        (
            size as u64,
            Header {
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

pub fn idstruct(input: &[u8]) -> IResult<&[u8], IdStruct> {
    let be_i32_pair = sequence::tuple((be_i32, be_i32));
    let (input, (version, machid, secs, usecs)) =
        sequence::tuple((be_i32, be_i32_pair, be_i32, be_i32))(input)?;

    Ok((
        input,
        IdStruct {
            version,
            machid,
            secs,
            usecs,
        },
    ))
}
use chrono::NaiveDateTime;

pub fn decode_unix_date(ivec: &Vec<i32>) -> String {
    NaiveDateTime::from_timestamp_opt(ivec[0].into(), ivec[1].try_into().unwrap())
        .unwrap()
        .to_string()
}

mod tests {
    use super::*;

    #[test]
    fn can_decode_date() {
        let data = Data::Int32(vec![1646226731, 138511]);

        if let Data::Int32(ivec) = data {
            assert_eq!(ivec.len(), 2);

            decode_unix_date(&ivec);
        }
    }
}
