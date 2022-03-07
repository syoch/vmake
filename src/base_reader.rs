pub use nom::bytes::complete::*;
use nom::IResult;

pub fn string(input: &[u8]) -> IResult<&[u8], String> {
    let (input, str) = take_until("\0")(input)?;
    let (input, _) = take(1usize)(input)?;

    Ok((input, String::from_utf8(str.to_vec()).unwrap()))
}

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
