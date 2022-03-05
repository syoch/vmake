use nom::bytes::complete::*;
use nom::error::ErrorKind;
use nom::number::complete::*;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub enum Entry {
    File(String, u64, Vec<u8>),
    Folder(String, u64, Vec<Entry>),
    RemoteResource(String, u64, String),
}

fn varint(input: &[u8]) -> IResult<&[u8], u64> {
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

fn read_string(input: &[u8]) -> IResult<&[u8], String> {
    let (input, length) = varint(input)?;
    let (input, string) = take(length)(input)?;

    Ok((input, String::from_utf8(string.to_vec()).unwrap()))
}

fn read_entry(input: &[u8]) -> IResult<&[u8], Entry> {
    let (input, t) = be_u8(input)?;
    let (input, name) = read_string(input)?;
    let (input, mtime) = be_u64(input)?;

    match t {
        0 => {
            let (input, size) = varint(input)?;
            let (input, data) = take(size)(input)?;

            Ok((input, Entry::File(name, mtime, data.to_vec())))
        }

        1 => {
            let (input, entry_count) = varint(input)?;

            let mut entries = Vec::new();

            let mut input = input;
            for _ in 0..entry_count {
                let tmp = read_entry(input)?;
                input = tmp.0;
                entries.push(tmp.1);
            }

            Ok((input, Entry::Folder(name, mtime, entries)))
        }

        2 => {
            let (input, href) = read_string(input)?;
            Ok((input, Entry::RemoteResource(name, mtime, href)))
        }

        _ => Err(nom::Err::Error(nom::error::Error::new(
            input,
            ErrorKind::Fail,
        ))),
    }
}

pub fn read_vmkp(input: &[u8]) -> IResult<&[u8], Entry> {
    let (input, _) = tag("vmkp")(input)?;

    Ok(read_entry(input)?)
}
