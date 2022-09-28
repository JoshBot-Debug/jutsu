use crate::{string_trail, segment::Find};

use super::byte;

const USERNAME_MAX_LENGTH: usize = 32;

#[derive(Debug)]
pub struct Username(pub String);

impl Username {
    pub fn new(find: &mut Find) -> Option<Self>
    {
        let entries = utmp_rs::parse_from_path("/var/run/utmp")
                .unwrap_or_else(|_| error("Failed to get user session".to_string()));
        for entry in entries {
            match entry {
                utmp_rs::UtmpEntry::UserProcess { user, .. } => {
                    if user.eq(&find.0) {
                        return Some(Self(string_trail(&mut find.0, USERNAME_MAX_LENGTH).to_owned()))
                    }
                }
                _ => {}
            }
        }
        None
    }

    pub fn buf(&self) -> Vec<u8>
    {
        let mut bytes = self.0.as_bytes().to_vec();
        let mut buf = vec![byte::USERNAME, 0, 0, self.0.len() as u8];
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

fn error(message: String) -> ! {
    eprintln!("{message}");
    std::process::exit(1)
}
