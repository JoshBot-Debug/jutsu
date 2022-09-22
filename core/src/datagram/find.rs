use super::Segment;


pub struct Find
{
    value: String
}

impl Segment for Find {

    fn data(&self) -> Vec<u8> {
        self.value.clone().into_bytes()
    }

    fn size(&self) -> Vec<u8> {
        
        vec![self.value.len().try_into().unwrap()]
    }
}

impl Find {
    pub fn new(value: String) -> Self
    {
        Self { value }
    }
}