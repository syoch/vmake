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

    pub fn resolve_entry(&self, inode: u64) -> Option<&Entry> {
        if let EntryData::Folder(entries) = &self.data {
            for entry in entries {
                if entry.inode == inode {
                    return Some(entry);
                }
            }
            return None;
        } else {
            return None;
        }
    }
}
