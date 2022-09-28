mod username;

use std::fmt::Error;

use crate::string_trail;

pub use self::username::Username;

use super::Segment;
use super::byte;

#[derive(Debug)]
pub struct Find(String);

const USERNAME_MAX_LENGTH: usize = 32;

impl Segment for Find {
    fn buf(&self) -> Vec<u8> {
        let mut bytes = self.0.as_bytes().to_vec();
        let mut buf = vec![byte::FIND, 0, bytes.len() as u8];
        buf.append(&mut bytes);
        return buf
    }
}

impl Find {
    pub fn new(value: &str) -> Self {
        Self(string_trail(&mut value.to_string(), USERNAME_MAX_LENGTH).to_owned())
    }

    pub fn result_from_buf(buf: &Vec<u8>) -> Option<Username> {
        if let Ok(mut find) = Self::from_buf(buf) {
            return Username::new(&mut find);
        }
        None
    }

    fn from_buf(buf: &Vec<u8>) -> Result<Self, Error> {
        if let Some(i) = buf.iter().enumerate().position(|(i, r)| {
            *r == byte::FIND
                && *buf
                    .get(i + 1)
                    .unwrap_or_else(|| error("Failed to parse buffer".to_string()))
                    == 0
        }) {
            if let Ok(value) = String::from_utf8(
                buf.get(
                    i + 3
                        ..i + 3
                            + *buf
                                .get(i + 2)
                                .unwrap_or_else(|| error("Failed to parse buffer".to_string()))
                                as usize,
                )
                .unwrap_or_else(|| error("Failed to parse buffer".to_string()))
                .to_vec(),
            ) {
                return Ok(Self(value));
            }
        }

        Err(Error)
    }
}

fn error(message: String) -> ! {
    eprintln!("{message}");
    std::process::exit(1)
}
