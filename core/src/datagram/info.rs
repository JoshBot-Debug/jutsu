use crate::{MemInfo, proc::{LoadAvg, Hostname}};

use super::Segment;

pub struct Info;

const INFO_BUF: u8 = 105;

impl Segment for Info {
    fn data(&self) -> Vec<u8> {
        vec![INFO_BUF, 0]
    }
}

impl Info {
    pub fn new() -> Self {
        Info
    }

    pub fn result_from_buf(buf: &Vec<u8>) -> Option<(MemInfo, LoadAvg, Hostname)> {
        
        if let Some(_) = buf.iter().enumerate().find(|(i, r)| {
            **r == INFO_BUF
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
