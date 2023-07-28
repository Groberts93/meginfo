use std::fs::File;
use std::io::{Cursor, Read, Seek, SeekFrom};


#[derive(Debug)]
struct Tag {
    kind: i32,  // DictionaryTags.txt
    type_: i32, // DictionaryTypes.txt
    size: i32,  // size in bytes
    next: i32,  // offset from start of file to next tag if > 0
}

impl Tag {
    fn new(ints: &[i32; 4]) -> Self {
        Tag {
            kind: ints[0],
            type_: ints[1],
            size: ints[2],
            next: ints[3],
        }
    }
}

fn main() {
    let mut fh = File::open("data/file_2.fif").unwrap();
    let mut buffer = [0; 2048];

    fh.read(&mut buffer).unwrap();

    let mut cursor = Cursor::new(buffer);

    for _ in 0..20 {
        let mut bytes = [0; 16];
        cursor.read(&mut bytes).unwrap();
        let (tag, next) = parse_tag_from_bytes(&bytes, cursor.position().try_into().unwrap());

        cursor.seek(SeekFrom::Start(next as u64)).unwrap();
        println!("{:#?}", tag);
    }
}


fn parse_tag_from_bytes(bytes: &[u8; 16], cursor: i32) -> (Tag, i32) {
    let mut ints = [0i32; 4];

    for (ii, chunk) in bytes.chunks_exact(4).enumerate() {
        let value = i32::from_be_bytes(chunk.try_into().unwrap());
        ints[ii] = value;
    }

    let next = match ints[3] {
        0 => ints[2] + cursor, // next tag is at cursor + size bytes
        _ => ints[3],          // next tag is at this absolute location
    };

    (Tag::new(&ints), next)
}
