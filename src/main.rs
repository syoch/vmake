mod vmkp;

// use fuse::*;
// use time::Timespec;
//
// const TTL: Timespec = Timespec { sec: 1, nsec: 0 };

fn main() {
    env_logger::init().expect("Failed to initialize logger");

    // let fs = Vmkp::new();
    // fuse::mount(fs, &"/run/user/1000/vmkp", &[]).expect("Failed to mount filesystem");
}
