use std::collections::HashMap;

use std::os::raw::c_int;

use fuse::*;
use time::Timespec;

use super::vmkp;

const TTL: Timespec = Timespec { sec: 1, nsec: 0 };

struct VmkpFS {
    vmkp: vmkp::Vmkp,
    inodes: HashMap<u64, String>, // inode : path
    next_available_inode: u64,
}

impl VmkpFS {
    fn new(vmkp: vmkp::Vmkp) -> VmkpFS {
        VmkpFS {
            vmkp: vmkp,
            inodes: HashMap::new(),
            next_available_inode: 1,
        }
    }

    fn register_inode(&mut self, path: String) {
        self.inodes.insert(self.next_available_inode, path);
        self.next_available_inode += 1;
    }

    fn register_all_inode(&mut self, entry: &vmkp::Entry, path: String) {
        if let vmkp::Data::Directory { entries: ref dir } = entry.data {
            for entry in dir {
                self.register_all_inode(entry, path.clone() + "/" + &entry.name);
            }
        } else {
            self.register_inode(path);
        }
    }
}

impl Filesystem for VmkpFS {
    fn init(&mut self, _req: &Request) -> Result<(), c_int> {
        self.register_all_inode(&self.vmkp.root, "/".to_string());
        Ok(())
    }
}

pub fn mount(mountpoint: String, vmkp: vmkp::Vmkp) {
    let fs = VmkpFS::new(vmkp);
    fuse::mount(fs, &mountpoint, &[]).expect("Failed to mount filesystem");
}
