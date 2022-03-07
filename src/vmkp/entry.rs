use nom::bytes::complete::*;
use nom::number::complete::*;

#[derive(Debug)]
pub struct Entry {
    pub name: String,
    pub mtime: u64,
    pub data: super::Data,
}

impl Entry {
    pub fn read(input: &[u8]) -> nom::IResult<&[u8], Entry> {
        let (input, t) = u8(input)?;
        let (input, name) = take_until("\0")(input)?;
        let (input, _) = take(1usize)(input)?;
        let (input, mtime) = be_u64(input)?;
        let (input, data) = super::Data::read(t, input)?;

        Ok((
            input,
            Entry {
                name: String::from_utf8(name.to_vec()).unwrap(),
                mtime,
                data,
            },
        ))
    }
}
