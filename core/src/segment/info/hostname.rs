use crate::string_trail;

use super::byte;

const HOSTNAME_MAX_LENGTH: usize = 32;

#[derive(Debug)]
pub struct Hostname(pub String);

impl Hostname {
    pub fn new() -> Self
    {
        let mut hostname = std::fs::read_to_string("/etc/hostname")
        .unwrap_or("unknown".to_string())
        .trim()
        .to_string();
        
        Self(string_trail(&mut hostname, HOSTNAME_MAX_LENGTH).to_owned())
    }

    pub fn buf(&self) -> Vec<u8>
    {
        let mut bytes = self.0.as_bytes().to_vec();
        let mut buf = vec![byte::HOSTNAME, 0, 0, bytes.len() as u8];
        buf.append(&mut bytes);
        return buf
    }

    pub fn from_buf(buf: &Vec<u8>) -> Self
    {
        if let Ok(username) = String::from_utf8(buf.clone())
        {
            return Self(username)
        }
        Self("unknown".to_string())
    }
}