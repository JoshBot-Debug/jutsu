
pub trait Segment {
    fn size(&self) -> Vec<u8>;    
    fn data(&self) -> Vec<u8>;    
}