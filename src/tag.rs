use serde::Deserialize;

#[derive(Debug)]
pub struct Tag {
    pub kind: i32, // DictionaryTags.txt
    type_: i32,    // DictionaryTypes.txt
    size: i32,     // size in bytes
    next: i32,     // offset from start of file to next tag if > 0
}

impl Tag {
    pub fn new(ints: &[i32; 4]) -> Self {
        Tag {
            kind: ints[0],
            type_: ints[1],
            size: ints[2],
            next: ints[3],
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
