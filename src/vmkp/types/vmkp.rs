#[derive(Debug)]
pub struct Vmkp {
    pub root: super::Entry,
}

impl Vmkp {
    pub fn new(root: super::Entry) -> Vmkp {
        Vmkp { root: root }
    }
}
