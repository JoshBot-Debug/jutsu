use std::fmt::Error;

use super::Segment;

#[derive(Debug)]
pub struct Find(String);

const FIND_BUF: u8 = 102;

impl Segment for Find {
    fn data(&self) -> Vec<u8> {
        let length = match u8::try_from(self.0.len()) {
            Ok(v) => {
                if v > 254 {
                    error("Find has too many characters. Maximum is 254".to_string())
                }
                v
            }
            Err(_) => error("Find has too many characters. Maximum is 254".to_string()),
        };

        let mut data = Vec::with_capacity(length.into());

        let mut buf = self.0.clone().into_bytes();

        data.append(&mut [FIND_BUF, 0, length].into());
        data.append(&mut buf);
        data
    }
}

impl Find {
    pub fn new(value: &str) -> Self {
        Self(value.to_string())
    }

    pub fn result_from_buf(buf: &Vec<u8>) -> Option<Vec<String>> {
        if let Ok(find) = Self::from_buf(buf) {
            let entries = utmp_rs::parse_from_path("/var/run/utmp")
                .unwrap_or_else(|_| error("Failed to get user session".to_string()));

            let mut result = Vec::with_capacity(5);

            entries.iter().for_each(|entry| match entry {
                utmp_rs::UtmpEntry::UserProcess { user, .. } => {
                    if user.contains(&find.0) {
                        result.push(user.clone())
                    }
                }
                _ => {}
            });

            if result.len() > 0
            {
                return Some(result);
            }
        }
        None
    }

    fn from_buf(buf: &Vec<u8>) -> Result<Self, Error> {
        if let Some(i) = buf.iter().enumerate().position(|(i, r)| {
            *r == FIND_BUF
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
