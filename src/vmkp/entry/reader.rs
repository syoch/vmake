use super::base_reader::*;
use super::data::EntryData;
use super::entry_type::Type;
use super::Entry;

use nom::{HexDisplay, IResult};

fn entry_data(input: &[u8], ino: u64, t: Type) -> IResult<&[u8], (u64, EntryData)> {
    let mut input = input;
    let data = match t {
        Type::File => {
            let (new_input, data) = take_until("\0")(input)?;
            input = new_input;
            let (new_input, _) = take(1usize)(input)?;
            input = new_input;

            EntryData::new_file(data.to_vec())
        }

        Type::Folder => {
            let mut entries = Vec::new();
            let mut ino = ino + 1;
            let mut input = input;
            while !input.starts_with(&[255]) {
                let (new_input, (next_ino, entry)) = entry(ino, input)?;
                input = new_input;
                entries.push(entry);
                ino = next_ino;
            }

            let (new_input, _) = take(1usize)(input)?;
            input = new_input;

            EntryData::new_folder(entries)
        }

        Type::RemoteResource => {
            let (new_input, href) = string(input)?;
            input = new_input;

            EntryData::new_remote_resource(href)
        }
    };

    return Ok((input, (ino + 1, data)));
}

pub fn entry(ino: u64, input: &[u8]) -> IResult<&[u8], (u64, Entry)> {
    let (input, t) = be_u8(input)?;
    let (input, name) = string(input)?;
    let (input, mtime) = be_u64(input)?;

    let (input, (_, entry_data)) = entry_data(input, ino, Type::from(t))?;

    Ok((input, (ino, Entry::new(name, ino, mtime, entry_data))))
}
