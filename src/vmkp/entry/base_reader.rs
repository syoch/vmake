pub use nom::bytes::complete::*;
use nom::IResult;

pub fn string(input: &[u8]) -> IResult<&[u8], String> {
    let (input, str) = take_until("\0")(input)?;
    let (input, _) = take(1usize)(input)?;

    Ok((input, String::from_utf8(str.to_vec()).unwrap()))
}
