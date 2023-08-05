use nom::number::complete::{be_i32, be_f32};
use nom::{sequence, IResult};
use nom::multi;

use crate::tag::TagHeader;

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