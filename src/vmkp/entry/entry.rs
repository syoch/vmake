use super::data::EntryData;
use fuse::FileAttr;
use fuse::FileType;
use time::Timespec;

#[derive(Debug)]
pub struct Entry {
    pub name: String,
    pub attr: FileAttr,

    pub data: EntryData,
}

impl Entry {
    pub fn fully_new(name: String, attr: FileAttr, data: EntryData) -> Entry {
        Entry { name, attr, data }
    }
    pub fn new(name: String, inode: u64, mtime: u64, data: EntryData) -> Entry {
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
            data,
        }
    }

    pub fn resolve_entry(&self, inode: u64) -> Option<&Entry> {
        if let EntryData::Folder(entries) = &self.data {
            for entry in entries {
                if entry.attr.ino == inode {
                    return Some(entry);
                }
            }
            return None;
        } else {
            return None;
        }
    }
}
