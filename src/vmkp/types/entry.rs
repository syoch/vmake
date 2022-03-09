#[derive(Debug)]
pub struct Entry {
    pub name: String,
    pub mtime: u64,
    pub data: super::Data,
}
