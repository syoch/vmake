mod vmkp;

use std::{ffi::OsStr, os::raw::c_int};

use fuse::*;
use libc::ENOENT;
use time::Timespec;
use vmkp::entry::EntryData;
use vmkp::Vmkp;

const TTL: Timespec = Timespec { sec: 1, nsec: 0 };

impl Filesystem for Vmkp {
    fn init(&mut self, _req: &Request) -> Result<(), c_int> {
        println!("Filesystem are");
        println!("{}", self.root);

        Ok(())
    }
    fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        let parent = self.root.resolve_entry(parent);

        if let None = parent {
            reply.error(ENOENT);
            return;
        }
        let parent = parent.unwrap();

        if let EntryData::Folder(entries) = &parent.data {
            for entry in entries {
                if entry.name == name.to_str().unwrap() {
                    reply.entry(&TTL, &entry.attr, 0);
                    return;
                }
            }
        }
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

    let fs = Vmkp::new();
    fuse::mount(fs, &"/run/user/1000/vmkp", &[]).expect("Failed to mount filesystem");
}
