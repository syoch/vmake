#[derive(Debug)]
pub enum Data {
    File { data: Vec<u8> },
    Directory { entries: Vec<super::Entry> },
    RemoteResource { uri: String },
}
