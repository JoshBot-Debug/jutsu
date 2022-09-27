use crate::string_limit;

type OneMinute = f64;
type FiveMinute = f64;
type FifteenMinute = f64;

#[derive(Debug)]
pub struct LoadAvg(OneMinute, FiveMinute, FifteenMinute);

const LOAD_AVG_BUF: u8 = 108;

impl LoadAvg {
    pub fn new() -> Self
    {
        let file = std::fs::read_to_string("/proc/loadavg")
        .unwrap_or("0.0 0.0 0.0 0/0 0".to_string());

        let mut line = file.split_whitespace();

        Self(
            Self::parse(&line.next()),
            Self::parse(&line.next()),
            Self::parse(&line.next()),
        )
    }

    pub fn buf(&self) -> Vec<u8>
    {
        let mut result: Vec<u8> = Vec::with_capacity(32);
        let default = String::from("0.0");

        let one: &mut Vec<u8> = &mut string_limit(&self.0.to_string(), 5).unwrap_or(&default).as_bytes().into();
        let five: &mut Vec<u8> = &mut string_limit(&self.1.to_string(), 5).unwrap_or(&default).as_bytes().into();
        let fifteen: &mut Vec<u8> = &mut string_limit(&self.2.to_string(), 5).unwrap_or(&default).as_bytes().into();

        result.append(&mut [LOAD_AVG_BUF, 49, 0, one.len() as u8].to_vec());
        result.append(one);
        result.append(&mut [LOAD_AVG_BUF, 53, 0, five.len() as u8].to_vec());
        result.append(five);
        result.append(&mut [LOAD_AVG_BUF, 49, 53, 0, fifteen.len() as u8].to_vec());
        result.append(fifteen);
        result
    }

    // pub fn from_buf(buf: &Vec<u8>) -> Result<Self, LoadAvgError> {

    //     result.append(&mut [LOAD_AVG_BUF, 49, 0, one.len() as u8].to_vec());
    //     result.append(one);
    //     result.append(&mut [LOAD_AVG_BUF, 53, 0, five.len() as u8].to_vec());
    //     result.append(five);
    //     result.append(&mut [LOAD_AVG_BUF, 49, 53, 0, fifteen.len() as u8].to_vec());
    //     result.append(fifteen);
    //     result
    // }

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
}

#[derive(Debug)]
pub struct LoadAvgError;

impl std::fmt::Display for LoadAvgError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse load average from buffer")
    }
}

fn error(message: String) -> ! {
    eprintln!("{message}");
    std::process::exit(1)
}