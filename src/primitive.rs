
#[derive(Debug)]
pub enum Primitive {
    Void,
    Int32(i32),
    Unknown,
}

pub fn parse_primitive(buffer: &[u8], code: u8) -> Primitive {
    match code {
        0 => Primitive::Void,
        3 => Primitive::Int32(i32::from_be_bytes(buffer.try_into().unwrap())),
        _ => Primitive::Unknown,
    }
}
