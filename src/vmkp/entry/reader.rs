use super::base_reader::*;
use super::entry_type::Type;
use super::Entry;

use nom::IResult;

pub fn entry(ino: u64, input: &[u8]) -> IResult<&[u8], (u64, Entry)> {
    let (input, t) = be_u8(input)?;
    let (input, name) = string(input)?;
    let (input, mtime) = be_u64(input)?;

    let (input, (_, entry_data)) = super::data::entry_data(input, ino, Type::from(t))?;

    Ok((input, (ino, Entry::new(name, ino, mtime, entry_data))))
}
