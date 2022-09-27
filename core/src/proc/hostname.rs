
#[derive(Debug)]
pub struct Hostname(String);

impl Hostname {
    pub fn new() -> Self
    {
        let hostname = std::fs::read_to_string("/etc/hostname")
        .unwrap_or("unknown".to_string())
        .trim()
        .to_string();
        
        Self(hostname)
    }
}