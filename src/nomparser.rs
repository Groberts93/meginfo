use nom::number::streaming::be_i32;
use nom::{sequence, IResult};

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
