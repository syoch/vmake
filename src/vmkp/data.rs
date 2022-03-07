use super::Entry;

pub enum Data {
    File { data: Vec<u8> },
    Directory { entries: Vec<Entry> },
    RemoteResource { uri: String },
}
