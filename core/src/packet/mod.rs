mod packet16;
mod execute;

pub use self::packet16::Packet16;
pub use self::execute::Execute;

pub trait Packet
{
    type Bytes;

    fn new(execute: Execute) -> Self;
    fn as_bytes(&self) -> Self::Bytes;
    fn from_buf(byte: &[u8; 16]) -> Self;
}