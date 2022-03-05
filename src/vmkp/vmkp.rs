use super::entry::*;
pub use nom::bytes::complete::*;
use nom::IResult;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct Vmkp {
    // last_inode: u64,
    root: Entry,
}

impl Vmkp {
    pub fn new() -> Vmkp {
        let mut file = File::open("test.vmkp").unwrap();
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();
        let (_, vmkp) = read_vmkp(&buf).unwrap();
        Vmkp {
            last_inode: 0,
            root: vmkp,
        }
    }

    // pub resolve_entry(mut &self, inode: u64) {
    //
    // }
}

pub fn read_vmkp(input: &[u8]) -> IResult<&[u8], Entry> {
    let (input, _) = tag("vmkp")(input)?;

    Ok(entry(input)?)
}
