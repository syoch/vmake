pub use nom::bytes::complete::*;
pub use nom::number::complete::*;
use nom::IResult;

pub fn varint(input: &[u8]) -> IResult<&[u8], u64> {
    let mut ret = 0;
    let mut shift = 0;
    let mut len = 0;
    for (_, b) in input.iter().enumerate() {
        len += 1;
        let b = *b as u64;
        let b = b & 0x7f;
        ret += b << shift;
        if b < 0x80 {
            break;
        }
        shift += 7;
    }
    Ok((&input[len..], ret))
}

pub fn string(input: &[u8]) -> IResult<&[u8], String> {
    let (input, length) = varint(input)?;
    let (input, string) = take(length)(input)?;

    Ok((input, String::from_utf8(string.to_vec()).unwrap()))
}
