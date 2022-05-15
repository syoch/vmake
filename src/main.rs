mod fuse;
mod vmkp;

use std::fs::File;
use std::io::Read;

fn main() {
    env_logger::init().expect("Failed to initialize logger");

    let mut file = File::open("test.vmkp").expect("Failed to open file");
    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("Failed to read file");

    let (_, vmkp) = vmkp::read(&*data).unwrap();

    println!("{:?}", vmkp);

    fuse::mount("/run/user/1000/vmkp".to_string(), vmkp);
}
