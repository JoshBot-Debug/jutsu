use std::str::Lines;

type MemTotal = f64;
type MemFree = f64;
type MemAvailable = f64;

#[derive(Debug)]
pub struct MemInfo(MemTotal, MemFree, MemAvailable);

impl MemInfo {
    pub fn new() -> Self {
        let file = std::fs::read_to_string("/proc/meminfo").unwrap_or("MemTotal: 0 kb".to_string());

        Self(
            Self::parse(&mut file.lines(), "MemTotal"),
            Self::parse(&mut file.lines(), "MemFree"),
            Self::parse(&mut file.lines(), "MemAvailable"),
        )
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
