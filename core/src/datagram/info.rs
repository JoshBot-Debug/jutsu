use super::Segment;

pub struct Info;

impl Segment for Info {
    fn data(&self) -> Vec<u8> {
        vec![4, 105, 110, 102, 111]
    }
}

impl Info {
    pub fn new() -> Self {
        Info
    }
}