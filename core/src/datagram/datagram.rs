use super::Segment;

#[derive(Debug)]
pub struct Datagram
{
    size: Vec<u8>,
    data: Vec<u8>,
    chunk: Vec<u8>,
}

impl Datagram
{
    pub fn new() -> Self
    {
        Self { size: vec![], data: vec![] }
    }

    pub fn push(&mut self, segment: Box<dyn Segment>)
    {
        self.data.append(&mut segment.data());
        self.size.append(&mut segment.size());
    }

    pub fn chunk(&self, size: u8) -> &[u8] 
    {
        let mut iter = self.data.chunks(size.into());

        print!("v {:?}", iter.);
        
        &self.data
    }
}