use std::fs::File;
use std::io::Read;

fn main() {
    let mut fh = File::open("data/file_2.fif").unwrap();
    let mut buffer = [0; 16];

    fh.read(&mut buffer).unwrap();

    // let value = i32::from_be_bytes(buffer[0..4].try_into().unwrap());

    let tag = parse_tag_from_bytes(&buffer);

    println!("{:?}", tag);
}

// struct Tag {
//     kind: i32, FIFF.FIFFB_MEAS = 100
//     type_: i32, FIFF.FIFFT_ID_STRUCT = 31
//     size: i32, 20
//     next: i32, 0
// }



fn parse_tag_from_bytes(bytes: &[u8]) -> [i32; 4] {
    let mut ints = [0i32; 4];

    for (ii, chunk) in bytes.chunks_exact(4).enumerate() {
        let value = i32::from_be_bytes(chunk.try_into().unwrap());
        ints[ii] = value;
    }
    ints
}
