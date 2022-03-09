#[derive(Debug)]
pub struct Vmkp {
    root: super::Entry,
}

impl Vmkp {
    pub fn new(root: super::Entry) -> Vmkp {
        Vmkp { root: root }
    }
}
