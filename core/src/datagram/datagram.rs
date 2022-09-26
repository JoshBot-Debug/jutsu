use super::Segment;

#[derive(Debug)]
pub struct Datagram {
    payload: Vec<u8>,
}

pub const DATAGRAM_SIZE: usize = 32;

impl Datagram {
    pub fn new() -> Self {
        Self {
            payload: Vec::with_capacity(DATAGRAM_SIZE),
        }
    }

    pub fn push(&mut self, segment: impl Segment) {
        self.payload.append(&mut segment.data());
    }

    pub fn buf(&mut self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.payload.len());
        buf.append(&mut self.payload);
        buf
    }

    // pub fn buf(&mut self) -> Vec<u8> {
    //     let mut size = size(self.payload.len());
    //     let mut buf = Vec::with_capacity(self.payload.len() + size.len());
    //     buf.append(&mut size);
    //     buf.append(&mut self.payload);
    //     buf
    // }
}

fn _size(length: usize) -> Vec<u8> {
    let size = ((length - 1) / 255) + 1;

    let mut result = Vec::with_capacity(size);

    for i in 1..size + 1 {
        if i < size {
            result.push(255);
            continue;
        }
        if let Ok(byte) = u8::try_from(length % 255) {
            if byte == 0 {
                result.push(255);
                continue;
            }
            result.push(byte);
        }
    }

    result.push(0);
    result
}
