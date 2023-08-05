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
enum Data {
    Slice(Vec<u8>),
    InFile { start: u64, size: u64 },
    Void,
    Byte(u8),
    Int16(i16),
    Int32(i32),
    Float(f32),
}

impl Data {
    pub fn from_slice(slice: Vec<u8>, dtype: i32) -> Self {
        let result = match dtype {
            0 => Data::Void,
            _ => Data::Slice(slice),
        };

        result
    }
}

#[derive(Debug)]
pub struct Tag {
    pub code: i32,
    pub dtype: i32,
    data: Data,
}

impl Tag {
    pub fn from_header_slice(header: TagHeader, slice: Vec<u8>) -> Self {
        Tag {
            code: header.code,
            dtype: header.dtype,
            data: Data::Slice(slice),
        }
    }

    pub fn from_header_file_position(header: TagHeader, start: u64, size: u64) -> Self {
        Tag {
            code: header.code,
            dtype: header.dtype,
            data: Data::InFile {
                start: start,
                size: size,
            },
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
