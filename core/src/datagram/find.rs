use crate::string_trail;

use super::Segment;

#[derive(Debug)]
pub struct Find(String);

const USERNAME_MAX_LENGTH: usize = 32;

const FIND_BUF: u8 = 102;

impl Segment for Find {
    fn buf(&self) -> Vec<u8> {
        let mut bytes = self.0.as_bytes().to_vec();
        let mut buf = vec![FIND_BUF, 0, bytes.len() as u8];
        buf.append(&mut bytes);
        return buf
    }
}

impl Find {
    pub fn new(value: &str) -> Self {
        Self(string_trail(&mut value.to_string(), USERNAME_MAX_LENGTH).to_owned())
    }

    pub fn result_from_buf(buf: &Vec<u8>) -> Option<(String, Vec<u8>)> {
        if let Ok(mut find) = Self::from_buf(buf) {
            let entries = utmp_rs::parse_from_path("/var/run/utmp")
                .unwrap_or_else(|_| error("Failed to get user session".to_string()));

            for entry in entries {
                match entry {
                    utmp_rs::UtmpEntry::UserProcess { user, .. } => {
                        if user.eq(&find.0) {
                            let username = string_trail(&mut find.0, USERNAME_MAX_LENGTH).to_owned();
                            let mut username_buf = vec![FIND_BUF, 0, username.len() as u8];
                            username_buf.append(&mut username.as_bytes().to_vec());
                            return Some((username, username_buf))
                        }
                    }
                    _ => {}
                }
            }
        }
        None
    }

    pub fn from_buf(buf: &Vec<u8>) -> Result<Self, FindError> {
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

        Err(FindError)
    }
}

#[derive(Debug)]
pub struct FindError;

impl std::fmt::Display for FindError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse find from buffer")
    }
}

fn error(message: String) -> ! {
    eprintln!("{message}");
    std::process::exit(1)
}
