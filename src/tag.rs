use serde::Deserialize;

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
#[derive(Debug)]
pub enum Data {
    Slice(Vec<u8>),
    InFile { start: u64, size: u64 },
}

#[derive(Debug)]
pub struct Tag {
    pub code: i32,
    pub dtype: i32,
    pub data: Data,
}

impl Tag {
    pub fn from_header(header: TagHeader, data: Data) -> Self {
        Tag {
            code: header.code,
            dtype: header.dtype,
            data,
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
