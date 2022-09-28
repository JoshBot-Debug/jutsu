use std::str::Lines;

use crate::string_limit;
use super::byte;

type MemTotal = f64;
type MemFree = f64;
type MemAvailable = f64;

#[derive(Debug)]
pub struct MemInfo(MemTotal, MemFree, MemAvailable);


impl MemInfo {
    pub fn new() -> Self {
        let file = std::fs::read_to_string("/proc/meminfo").unwrap_or("".to_string());

        Self(
            parse(&mut file.lines(), "MemTotal"),
            parse(&mut file.lines(), "MemFree"),
            parse(&mut file.lines(), "MemAvailable"),
        )
    }

    pub fn buf(&self) -> Vec<u8>
    {
        let mut result: Vec<u8> = Vec::with_capacity(32);
        let default = String::from("0.0");

        let total: &mut Vec<u8> = &mut string_limit(&self.0.to_string(), 5).unwrap_or(&default).as_bytes().into();
        let free: &mut Vec<u8> = &mut string_limit(&self.1.to_string(), 5).unwrap_or(&default).as_bytes().into();
        let available: &mut Vec<u8> = &mut string_limit(&self.2.to_string(), 5).unwrap_or(&default).as_bytes().into();

        result.append(&mut [byte::MEMINFO, 0, 1, total.len() as u8].to_vec());
        result.append(total);
        result.append(&mut [byte::MEMINFO, 0, 2, free.len() as u8].to_vec());
        result.append(free);
        result.append(&mut [byte::MEMINFO, 0, 3, available.len() as u8].to_vec());
        result.append(available);
        result
    }

    pub fn from_buf(buf: &Vec<u8>) -> String
    {
        if let Ok(mem) = String::from_utf8(buf.clone())
        {
            return mem;
        }
        "0.0".to_string()
    }
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