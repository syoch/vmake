mod vmkp;

use std::fs::File;
use std::io::Read;

// use fuse::*;
// use time::Timespec;
//
// const TTL: Timespec = Timespec { sec: 1, nsec: 0 };

fn main() {
    env_logger::init().expect("Failed to initialize logger");

    let mut file = File::open("test.vmkp").expect("Failed to open file");
    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("Failed to read file");

    let (_, vmkp) = vmkp::Vmkp::read(&*data).unwrap();

    println!("{:?}", vmkp);

    // let fs = Vmkp::new();
    // fuse::mount(fs, &"/run/user/1000/vmkp", &[]).expect("Failed to mount filesystem");
}
