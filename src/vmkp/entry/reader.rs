use super::base_reader::*;
use super::data::EntryData;
use super::entry_type::Type;
use super::Entry;

use nom::IResult;

fn entry_data(input: &[u8], last_inode: u64, t: Type) -> IResult<&[u8], (u64, EntryData)> {
    match t {
        Type::File => {
            let (input, size) = varint(input)?;
            let (input, data) = take(size)(input)?;

            Ok((input, (last_inode, EntryData::new_file(data.to_vec()))))
        }

        Type::Folder => {
            let (input, entry_count) = varint(input)?;

            let mut entries = Vec::new();
            let mut last_inode = last_inode;
            let mut input = input;
            for _ in 0..entry_count {
                let tmp = entry(last_inode, input)?;
                input = tmp.0;
                last_inode = tmp.1 .0;
                entries.push(tmp.1 .1);
            }

            Ok((input, (last_inode, EntryData::new_folder(entries))))
        }

        Type::RemoteResource => {
            let (input, href) = string(input)?;

            Ok((input, (last_inode, EntryData::new_remote_resource(href))))
        }
    }
}

pub fn entry(last_inode: u64, input: &[u8]) -> IResult<&[u8], (u64, Entry)> {
    let (input, t) = be_u8(input)?;
    let (input, name) = string(input)?;
    let (input, mtime) = be_u64(input)?;

    let (input, (inode, entry_data)) = entry_data(input, last_inode, Type::from(t))?;

    Ok((
        input,
        (last_inode, Entry::new(name, inode, mtime, entry_data)),
    ))
}
