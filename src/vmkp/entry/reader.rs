use super::base_reader::*;
use super::data::EntryData;
use super::entry_type::Type;
use super::Entry;

use nom::error::ErrorKind;
use nom::IResult;

fn entry_data(input: &[u8], t: Type) -> IResult<&[u8], EntryData> {
    match t {
        Type::File => {
            let (input, size) = varint(input)?;
            let (input, data) = take(size)(input)?;

            Ok((input, EntryData::new_file(data.to_vec())))
        }

        Type::Folder => {
            let (input, entry_count) = varint(input)?;

            let mut entries = Vec::new();

            let mut input = input;
            for _ in 0..entry_count {
                let tmp = entry(input)?;
                input = tmp.0;
                entries.push(tmp.1);
            }

            Ok((input, EntryData::new_folder(entries)))
        }

        Type::RemoteResource => {
            let (input, href) = string(input)?;

            Ok((input, EntryData::new_remote_resource(href)))
        }
    }
}

pub fn entry(input: &[u8]) -> IResult<&[u8], Entry> {
    let (input, t) = be_u8(input)?;
    let (input, name) = string(input)?;
    let (input, mtime) = be_u64(input)?;

    let (input, entry_data) = entry_data(input, Type::from(t))?;

    Ok((input, Entry::new(name, 0, mtime, entry_data)))
}
