mod vmkp;

use fuse::{FileAttr, Filesystem};
use time::Timespec;

struct FSEntry {
    inode: u64,
    attr: FileAttr,
    name: String,

    parent: u64,
    children: Vec<u64>,

    data: Vec<u8>,
}

// const TTL: Timespec = Timespec { sec: 1, nsec: 0 };

impl Filesystem for vmkp::Vmkp {
    // fn getattr(&mut self, _req: &fuse::Request, ino: u64, reply: fuse::ReplyAttr) {
    //     match self.entries.get(&ino) {
    //         Some(x) => {
    //             reply.attr(&TTL, &x.attr);
    //             return;
    //         }
    //         None => {
    //             reply.error(libc::ENOENT);
    //             return;
    //         }
    //     };
    // }
    // fn readdir(
    //     &mut self,
    //     _req: &fuse::Request,
    //     ino: u64,
    //     fh: u64,
    //     offset: i64,
    //     mut reply: fuse::ReplyDirectory,
    // ) {
    //     if offset == 0 {
    //         let ent = self.get_entry(ino).unwrap();
    //         for child in ent.children.clone() {
    //             println!("{}", child);
    //             let child_ent = self.get_entry(child).unwrap();
    //             reply.add(
    //                 child_ent.inode,
    //                 0,
    //                 fuse::FileType::RegularFile,
    //                 &child_ent.name,
    //             );
    //         }
    //     }
    //     reply.ok();
    // }
}

fn main() {
    env_logger::init().expect("Failed to initialize logger");

    fuse::mount(vmkp::vmkp::Vmkp::new(), &"vmkp", &[]).expect("Failed to mount filesystem");
}
