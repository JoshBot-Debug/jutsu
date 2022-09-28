use crate::string_limit;

use super::byte;

type OneMinute = f64;
type FiveMinute = f64;
type FifteenMinute = f64;

#[derive(Debug)]
pub struct LoadAvg(OneMinute, FiveMinute, FifteenMinute);

impl LoadAvg {
    pub fn new() -> Self
    {
        let file = std::fs::read_to_string("/proc/loadavg")
        .unwrap_or("0.0 0.0 0.0 0/0 0".to_string());

        let mut line = file.split_whitespace();

        Self(
            parse(&line.next()),
            parse(&line.next()),
            parse(&line.next()),
        )
    }

    pub fn buf(&self) -> Vec<u8>
    {
        let mut result: Vec<u8> = Vec::with_capacity(32);
        let default = String::from("0.0");

        let one: &mut Vec<u8> = &mut string_limit(&self.0.to_string(), 5).unwrap_or(&default).as_bytes().into();
        let five: &mut Vec<u8> = &mut string_limit(&self.1.to_string(), 5).unwrap_or(&default).as_bytes().into();
        let fifteen: &mut Vec<u8> = &mut string_limit(&self.2.to_string(), 5).unwrap_or(&default).as_bytes().into();

        result.append(&mut [byte::LOAD_AVG, 0, 1, one.len() as u8].to_vec());
        result.append(one);
        result.append(&mut [byte::LOAD_AVG, 0, 2, five.len() as u8].to_vec());
        result.append(five);
        result.append(&mut [byte::LOAD_AVG, 0, 3, fifteen.len() as u8].to_vec());
        result.append(fifteen);
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


fn parse(line: &Option<&str>) -> f64
{
    match line {
        Some(line) => {
            line
            .parse::<f64>()
            .unwrap_or(0.0)
            .to_owned()
        },
        None => 0.0
    }
}