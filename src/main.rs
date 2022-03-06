mod vmkp;

use std::os::raw::c_int;

use fuse::*;
use time::Timespec;
use vmkp::entry::EntryData;

const TTL: Timespec = Timespec { sec: 1, nsec: 0 };

impl Filesystem for vmkp::Vmkp {
    fn init(&mut self, _req: &Request) -> Result<(), c_int> {
        println!("Filesystem are");
        println!("{}", self.root);

        Ok(())
    }
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
                let mut i = 0;
                for entry in entries {
                    reply.add(entry.attr.ino, i, entry.attr.kind, &entry.name);

                    i += 1;
                }
            }
        }
        reply.ok();
    }
}

fn main() {
    env_logger::init().expect("Failed to initialize logger");

    let fs = vmkp::vmkp::Vmkp::new();
    fuse::mount(fs, &"/run/user/1000/vmkp", &[]).expect("Failed to mount filesystem");
}
