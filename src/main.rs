use std::fs::File;
use std::io::{self, Read};

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

fn main() -> io::Result<()> {
    let fh = File::open("data/file_0.fif").unwrap();
    let mut reader = io::BufReader::new(fh);
    let mut buf = [0u8; 16];

    while let Ok(()) = reader.read_exact(&mut buf) {
        let (tag, size) = parse_tag_from_bytes(&buf);
        println!("{:?}", tag);
        reader.seek_relative(size)?;
    }

    println!("Finished reading");

    Ok(())
}

fn parse_tag_from_bytes(bytes: &[u8; 16]) -> (Tag, i64) {
    let mut ints = [0i32; 4];

    for (ii, chunk) in bytes.chunks_exact(4).enumerate() {
        let value = i32::from_be_bytes(chunk.try_into().unwrap());
        ints[ii] = value;
    }
    (Tag::new(&ints), ints[2].try_into().unwrap())
}
