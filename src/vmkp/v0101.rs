use super::{Data, Entry, Vmkp};
use nom::bytes::complete::*;
use nom::number::complete::*;

pub fn varint(input: &[u8]) -> nom::IResult<&[u8], u64> {
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
impl Data {
    pub fn read_v0101(t: u8, input: &[u8]) -> nom::IResult<&[u8], Data> {
        match t {
            0 => {
                let (input, length) = varint(input)?;
                let (input, data) = take(length as usize)(input)?;

                Ok((
                    input,
                    Data::File {
                        data: data.to_vec(),
                    },
                ))
            }
            1 => {
                let mut entries = Vec::new();

                let mut input = input;
                while !input.starts_with(&[0xFF]) {
                    let (new_input, entry) = super::Entry::read_v0101(input)?;
                    input = new_input;
                    entries.push(entry);
                }

                let (input, _) = take(1usize)(input)?;

                Ok((input, Data::Directory { entries }))
            }
            2 => {
                let (input, uri) = take_until("\0")(input)?;

                Ok((
                    input,
                    Data::RemoteResource {
                        uri: String::from_utf8(uri.to_vec()).unwrap(),
                    },
                ))
            }
            _ => panic!("Unknown data type: {}", t),
        }
    }
}

impl Entry {
    pub fn read_v0101(input: &[u8]) -> nom::IResult<&[u8], Entry> {
        let (input, t) = u8(input)?;
        let (input, name) = take_until("\0")(input)?;
        let (input, _) = take(1usize)(input)?;
        let (input, mtime) = be_u64(input)?;
        let (input, data) = Data::read_v0101(t, input)?;

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

impl Vmkp {
    pub fn read(input: &[u8]) -> nom::IResult<&[u8], Vmkp> {
        let (input, _) = tag("vmkp")(input)?;
        let (input, version) = be_u16(input)?;

        let (input, _) = be_u16(input)?;

        let mut input = input;
        let root: Entry;
        if version == 0x0101 {
            let tmp = super::v0101::Entry::read_v0101(input)?;
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
}
