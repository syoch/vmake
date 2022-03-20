use super::{Entry, Vmkp};
use nom::bytes::complete::*;
use nom::number::complete::*;

pub fn read_vmkp(input: &[u8]) -> nom::IResult<&[u8], Vmkp> {
    let (input, _) = tag("vmkp")(input)?;
    let (input, version) = be_u16(input)?;

    let (input, _) = be_u16(input)?;

    let mut input = input;
    let root: Entry;

    if version == 0x0101 {
        let tmp = super::reader_0101::read_entry(input)?;
        input = tmp.0;
        root = tmp.1;
    } else {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }

    Ok((input, Vmkp::new(root)))
}
