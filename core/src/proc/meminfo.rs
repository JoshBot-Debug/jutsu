use std::str::Lines;

use crate::string_limit;

type MemTotal = f64;
type MemFree = f64;
type MemAvailable = f64;

#[derive(Debug)]
pub struct MemInfo(MemTotal, MemFree, MemAvailable);

const MEMINFO_BUF: u8 = 109;

impl MemInfo {
    pub fn new() -> Self {
        let file = std::fs::read_to_string("/proc/meminfo").unwrap_or("MemTotal: 0 kb".to_string());

        Self(
            Self::parse(&mut file.lines(), "MemTotal"),
            Self::parse(&mut file.lines(), "MemFree"),
            Self::parse(&mut file.lines(), "MemAvailable"),
        )
    }

    pub fn buf(&self) -> Vec<u8>
    {
        let mut result: Vec<u8> = Vec::with_capacity(32);
        let default = String::from("0.0");

        let total: &mut Vec<u8> = &mut string_limit(&self.0.to_string(), 5).unwrap_or(&default).as_bytes().into();
        let free: &mut Vec<u8> = &mut string_limit(&self.1.to_string(), 5).unwrap_or(&default).as_bytes().into();
        let available: &mut Vec<u8> = &mut string_limit(&self.2.to_string(), 5).unwrap_or(&default).as_bytes().into();

        result.append(&mut [MEMINFO_BUF, 116, 0, total.len() as u8].to_vec());
        result.append(total);
        result.append(&mut [MEMINFO_BUF, 102, 0, free.len() as u8].to_vec());
        result.append(free);
        result.append(&mut [MEMINFO_BUF, 97, 0, available.len() as u8].to_vec());
        result.append(available);
        result
    }

    fn parse(line: &mut Lines, key: &str) -> f64 {
        let line = line.find(|l| l.contains(key));

        match line {
            Some(line) => {
                let mut iter = line.split_whitespace();
                iter.next();

                f64::trunc(
                    (iter.next().unwrap_or("0.0").parse::<f64>().unwrap_or(0.0) / 1000000.0)
                        * 100.0,
                ) / 100.0
            }
            None => 0.0,
        }
    }
}
