use std::collections::HashMap;

use super::vmkp;

use fuse::*;
use time::Timespec;

const TTL: Timespec = Timespec { sec: 1, nsec: 0 };

struct VmkpFS {
    vmkp: vmkp::Vmkp,
    inodes: HashMap<u64, String>, // inode : path
}

impl VmkpFS {
    fn new(vmkp: vmkp::Vmkp) -> VmkpFS {
        VmkpFS {
            vmkp: vmkp,
            inodes: HashMap::new(),
        }
    }
}

impl Filesystem for VmkpFS {
    fn getattr(&mut self, req: &Request, ino: u64, reply: ReplyAttr) {}
}

pub fn mount(mountpoint: String, vmkp: vmkp::Vmkp) {
    let fs = VmkpFS::new(vmkp);
    fuse::mount(fs, &mountpoint, &[]).expect("Failed to mount filesystem");
}
