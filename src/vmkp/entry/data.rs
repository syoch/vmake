use super::entry::Entry;

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
