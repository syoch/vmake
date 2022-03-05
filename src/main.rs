mod vmkp;

use fuse::{FileAttr, Filesystem};
use std::collections::hash_map::HashMap;
use std::fs::File;
use std::io::Read;
use time::Timespec;

struct FSEntry {
    inode: u64,
    attr: FileAttr,
    name: String,

    parent: u64,
    children: Vec<u64>,

    data: Vec<u8>,
}

struct VmkpFS {
    vmkp: vmkp::Entry,
    entries: HashMap<u64, FSEntry>,
}

const TTL: Timespec = Timespec { sec: 1, nsec: 0 };

impl Filesystem for VmkpFS {
    fn getattr(&mut self, _req: &fuse::Request, ino: u64, reply: fuse::ReplyAttr) {
        match self.entries.get(&ino) {
            Some(x) => {
                reply.attr(&TTL, &x.attr);
                return;
            }
            None => {
                reply.error(libc::ENOENT);
                return;
            }
        };
    }
}

impl VmkpFS {
    fn new() -> VmkpFS {
        let mut file = File::open("test.vmkp").unwrap();
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();
        let (_, vmkp) = vmkp::read_vmkp(&buf).unwrap();
        VmkpFS {
            vmkp,
            entries: HashMap::new(),
        }
    }
}

fn main() {
    env_logger::init().expect("Failed to initialize logger");

    fuse::mount(VmkpFS::new(), &"vmkp", &[]).expect("Failed to mount filesystem");
}
