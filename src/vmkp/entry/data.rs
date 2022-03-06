use super::entry::Entry;
use crate::vmkp::entry::base_reader::*;
use crate::vmkp::entry::entry;
use crate::vmkp::entry::entry_type::Type;
use nom::IResult;

#[derive(Debug)]
pub enum EntryData {
    File(Vec<u8>),
    Folder(Vec<Entry>),
    RemoteResource(String),
}

impl EntryData {
    pub fn new_file(data: Vec<u8>) -> EntryData {
        EntryData::File(data)
    }

    pub fn new_folder(entries: Vec<Entry>) -> EntryData {
        EntryData::Folder(entries)
    }

    pub fn new_remote_resource(url: String) -> EntryData {
        EntryData::RemoteResource(url)
    }
}

pub fn entry_data(input: &[u8], mut ino: u64, t: Type) -> IResult<&[u8], (u64, EntryData)> {
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
            ino = ino + 1;
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
