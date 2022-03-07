use std::fmt::Display;

use super::base_reader::*;
use super::data::EntryData;
use super::entry_type::Type;
use fuse::FileAttr;
use fuse::FileType;

pub use nom::number::complete::*;
use nom::IResult;

#[derive(Debug)]
pub struct Entry {
    pub name: String,
    pub attr: FileAttr,

    pub parent_inode: u64,

    pub data: EntryData,
}

impl Entry {
    pub fn brief_to_string(&self) -> String {
        format!(
            "[{}::{} {:?}] '{}'",
            self.parent_inode, self.attr.ino, self.attr.kind, self.name
        )
    }

    fn to_string_multiline(&self) -> Vec<String> {
        let mut lines = Vec::new();

        lines.push(self.brief_to_string());

        if let EntryData::Folder(entries) = &self.data {
            for entry in entries {
                for line in entry.to_string_multiline() {
                    lines.push(format!("|   {}", line));
                }
            }
        }

        lines
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string_multiline().join("\n"))?;

        Ok(())
    }
}

impl Entry {
    pub fn new(name: String, inode: u64, parent_inode: u64, mtime: u64, data: EntryData) -> Entry {
        let kind = match data {
            EntryData::File(_) => FileType::RegularFile,
            EntryData::Folder(_) => FileType::Directory,
            EntryData::RemoteResource(_) => FileType::RegularFile,
        };

        let mtime = time::Timespec {
            sec: mtime as i64,
            nsec: 0,
        };

        Entry {
            name,
            attr: FileAttr {
                ino: inode,
                size: 0,
                blocks: 0,
                atime: mtime,
                mtime,
                ctime: mtime,
                crtime: mtime,
                kind,
                perm: 0o777,
                nlink: 1,
                uid: 0,
                gid: 0,
                rdev: 0,
                flags: 0,
            },
            parent_inode,
            data,
        }
    }

    pub fn resolve_entry(&self, inode: u64) -> Option<&Entry> {
        if self.attr.ino == inode {
            return Some(self);
        }

        if let EntryData::Folder(entries) = &self.data {
            for entry in entries {
                if let Some(x) = entry.resolve_entry(inode) {
                    return Some(x);
                }
            }
            return None;
        } else {
            return None;
        }
    }
}

pub fn entry(parent_inode: u64, ino: u64, input: &[u8]) -> IResult<&[u8], (u64, Entry)> {
    let (input, t) = Type::read(input)?;
    let (input, name) = string(input)?;
    let (input, mtime) = be_u64(input)?;

    let (input, (_, entry_data)) = super::data::entry_data(input, ino, t)?;

    Ok((
        input,
        (
            ino + 1,
            Entry::new(name, ino, parent_inode, mtime, entry_data),
        ),
    ))
}
