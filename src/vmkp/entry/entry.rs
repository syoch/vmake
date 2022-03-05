use super::data::EntryData;

#[derive(Debug, PartialEq)]
pub struct Entry {
    pub name: String,
    pub inode: u64,
    pub mtime: u64,

    pub data: EntryData,
}

impl Entry {
    pub fn new(name: String, inode: u64, mtime: u64, data: EntryData) -> Entry {
        Entry {
            name,
            inode,
            mtime,
            data,
        }
    }
}
