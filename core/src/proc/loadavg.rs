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
            Self::parse(&line.next()),
            Self::parse(&line.next()),
            Self::parse(&line.next()),
        )
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
}