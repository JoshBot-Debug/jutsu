use super::Segment;

pub struct Info;

impl Segment for Info {
    fn data(&self) -> Vec<u8> {
        vec![105,0]
    }
}

impl Info {
    pub fn new() -> Self {
        Info
    }
}