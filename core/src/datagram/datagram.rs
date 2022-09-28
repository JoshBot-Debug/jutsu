use crate::segment::byte;

use super::super::segment::Segment;

#[derive(Debug)]
pub struct Datagram {
    payload: Vec<u8>,
}

impl Datagram {
    pub fn new() -> Self {
        Self {
            payload: Vec::with_capacity(byte::DATAGRAM_SIZE),
        }
    }

    pub fn push(&mut self, segment: impl Segment) {
        self.payload.append(&mut segment.buf());
    }

    pub fn buf(&mut self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.payload.len());
        buf.append(&mut self.payload);
        buf
    }
}