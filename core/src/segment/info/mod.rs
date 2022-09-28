mod hostname;
mod loadavg;
mod meminfo;

pub use hostname::Hostname;
pub use loadavg::LoadAvg;
pub use meminfo::MemInfo;

use super::Segment;
use super::byte;


pub struct Info;

impl Segment for Info {
    fn buf(&self) -> Vec<u8> {
        vec![byte::INFO, 0, 0]
    }
}

impl Info {
    pub fn new() -> Self {
        Info
    }

    pub fn result_from_buf(buf: &Vec<u8>) -> Option<(MemInfo, LoadAvg, Hostname)> {
        
        if let Some(_) = buf.iter().enumerate().find(|(i, r)| {
            **r == byte::INFO
                && *buf
                    .get(i + 1)
                    .unwrap_or_else(|| error("Failed to parse buffer".to_string()))
                    == 0
        }) {
            return Some((MemInfo::new(), LoadAvg::new(), Hostname::new()));
        };

        None
    }
}

fn error(message: String) -> ! {
    eprintln!("{message}");
    std::process::exit(1)
}
