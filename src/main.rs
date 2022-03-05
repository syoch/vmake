mod vmkp;

use fuse::*;
use time::Timespec;
use vmkp::entry::EntryData;

struct FSEntry {
    inode: u64,
    attr: FileAttr,
    name: String,

    parent: u64,
    children: Vec<u64>,

    data: Vec<u8>,
}

const TTL: Timespec = Timespec { sec: 1, nsec: 0 };

impl Filesystem for vmkp::Vmkp {
    fn getattr(&mut self, _req: &fuse::Request, ino: u64, reply: fuse::ReplyAttr) {
        match self.root.resolve_entry(ino) {
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
    fn readdir(
        &mut self,
        _req: &fuse::Request,
        ino: u64,
        _fh: u64,
        offset: i64,
        mut reply: fuse::ReplyDirectory,
    ) {
        if offset == 0 {
            let ent = self.root.resolve_entry(ino).unwrap();
            if let EntryData::Folder(entries) = &ent.data {
                for entry in entries {
                    reply.add(entry.attr.ino, 0, entry.attr.kind, &entry.name);
                }
            }
        }
        reply.ok();
    }
}

fn main() {
    env_logger::init().expect("Failed to initialize logger");

    fuse::mount(vmkp::vmkp::Vmkp::new(), &"vmkp", &[]).expect("Failed to mount filesystem");
}
