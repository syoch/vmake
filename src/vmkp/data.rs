use nom::bytes::complete::*;

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

#[derive(Debug)]
pub enum Data {
    File { data: Vec<u8> },
    Directory { entries: Vec<super::Entry> },
    RemoteResource { uri: String },
}

impl Data {
    pub fn read(t: u8, input: &[u8]) -> nom::IResult<&[u8], Data> {
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
                    let (new_input, entry) = super::Entry::read(input)?;
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
