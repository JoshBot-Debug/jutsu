use serde::{Serialize, Deserialize};
use bincode;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Response
{
    hostname: Option<String>,
    session: Vec<String>,
}

impl Response
{
    pub fn new() -> Self
    {
        let hostname = match std::fs::read_to_string("/etc/hostname") {
            Ok(hostname) => Some(hostname.trim().to_string()),
            Err(_) => None
        };
        
        Self { hostname, session: Vec::new() }
    }

    pub fn buf(&self) -> Result<Vec<u8>, String>
    {
        match bincode::serialize(&self) {
            Ok(bytes) => Ok(bytes),
            Err(_) => Err(String::from("Failed to serialize response."))
        }
    }

    pub fn from_buf(buf: &[u8]) -> Result<Self, String>
    {
        match bincode::deserialize(buf) {
            Ok(bytes) => Ok(bytes),
            Err(_) => Err(String::from("Failed to deserialize response."))
        }
    }

    pub fn session_includes(&self, username: &String) -> bool
    {
        self.session.contains(username)
    }

    pub fn print_session(&self)
    {
        println!("Session:   {}", self.session.join(","));
    }

    pub fn print_hostname(&self)
    {
        if let Some(hostname) = &self.hostname
        {
            println!("Hostname:  {}", hostname);
        }
    }

    pub fn refresh(&mut self) -> &Self
    {
        self.session.clear();
        match utmp_rs::parse_from_path("/var/run/utmp") {
            Ok(entries) =>
            {
                for entry in entries {
                    match entry {
                        utmp_rs::UtmpEntry::UserProcess { user, .. } => {
                            self.session.push(user)
                        }
                        _ => {}
                    }
                }
            },
            Err(_) => {}
        }

        self
    }
}