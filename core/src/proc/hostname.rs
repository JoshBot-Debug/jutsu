use crate::string_trail;

const HOSTNAME_MAX_LENGTH: usize = 32;

const HOSTNAME_BUF: u8 = 104;

#[derive(Debug)]
pub struct Hostname(String);

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
        let mut buf = vec![HOSTNAME_BUF, 0, bytes.len() as u8];
        buf.append(&mut bytes);
        return buf
    }

    pub fn from_buf(buf: &Vec<u8>) -> Result<Self, HostnameError> {
        if let Some(i) = buf.iter().enumerate().position(|(i, r)| {
            *r == HOSTNAME_BUF
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

        Err(HostnameError)
    }
}

#[derive(Debug)]
pub struct HostnameError;

impl std::fmt::Display for HostnameError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse hostname from buffer")
    }
}

fn error(message: String) -> ! {
    eprintln!("{message}");
    std::process::exit(1)
}