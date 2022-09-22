use std::process;

use super::{Packet, Execute};

#[derive(Debug)]
pub struct Packet16
{
    pub execute: Execute,
}

impl Packet for Packet16 {
    type Bytes = [u8; 16];

    fn new(execute: Execute) -> Self {
        Self { execute }
    }

    fn as_bytes(&self) -> Self::Bytes {
        let mut buf = [0; 16];

        for (i, b) in buf.iter_mut().enumerate() {
            if i < 2 {
                *b = *self.execute.to_byte();
                continue;
            };
        }

        buf
    }

    fn from_buf(byte: &[u8; 16]) -> Self {
        let execute = match Execute::from_byte(&byte[..2]) {
            Ok(v) => v,
            Err(e) =>
            {
                eprintln!("{e}");
                process::exit(e.code)
            }
        };
        Self { execute }
    }
}


#[cfg(test)]
mod tests {

    use crate::{Packet, Packet16, Execute};

    #[test]
    fn enum_execute_restart_from_buf() {
        let buff = &mut [0; 16];
        buff[0] = *Execute::Restart.to_byte();
        let packet: Packet16 = Packet16::from_buf(buff);

        assert!(matches!(packet.execute, Execute::Restart));
    }

    #[test]
    fn enum_execute_shutdown_from_buf() {
        let buff = &mut [0; 16];
        buff[0] = *Execute::Shutdown.to_byte();
        let packet: Packet16 = Packet16::from_buf(buff);

        assert!(matches!(packet.execute, Execute::Shutdown));
    }

    #[test]
    fn packet_as_bytes() {
        let packet = Packet16::new(Execute::Restart);
        assert_eq!(packet.as_bytes().len(), 16);
    }

}