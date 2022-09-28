mod info;
mod find;

pub use info::{Info, Hostname, MemInfo, LoadAvg};
pub use find::{Find, Username};

pub trait Segment {
    fn buf(&self) -> Vec<u8>;
}

impl std::fmt::Debug for dyn Segment
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "segment stuff")
    }
}

pub fn output_from_buf(buf: &Vec<u8>) -> String
{
    let mut output = String::new();


    for (index, byte) in buf.iter().enumerate()
    {
        match (
            byte,
            buf.get(index + 1).unwrap_or(&255),
            buf.get(index + 2).unwrap_or(&255),
            buf.get(index + 3).unwrap_or(&255),
        ) {
            (&byte::USERNAME, 0, _, len) => {
                let username = Username::from_buf(&buf[index+4..(index+4+*len as usize)].to_vec());
                output.push_str(format!("\nUsername:           {}", username.0).as_str());
            },
            (&byte::HOSTNAME, 0, _, len) => {
                let hostname = Hostname::from_buf(&buf[index+4..(index+4+*len as usize)].to_vec());
                output.push_str(format!("\nHostname:           {}", hostname.0).as_str());
            },
            (&byte::LOAD_AVG, 0, c, len) => {
                let percent = LoadAvg::from_buf(&buf[index+4..(index+4+*len as usize)].to_vec());
                match c {
                    1 => output.push_str(format!("\nCPU load avg (1m):  {}%", percent).as_str()),
                    2 => output.push_str(format!("\nCPU load avg (5m):  {}%", percent).as_str()),
                    3 => output.push_str(format!("\nCPU load avg (15m): {}%", percent).as_str()),
                    _ => output.push_str(format!("\nCPU load Avg (unknown): {}%", percent).as_str()),
                }
            },
            (&byte::MEMINFO, 0, c, len) => {
                let gb = MemInfo::from_buf(&buf[index+4..(index+4+*len as usize)].to_vec());
                match c {
                    1 => output.push_str(format!("\nRAM (total):        {} GB", gb).as_str()),
                    2 => output.push_str(format!("\nRAM (free):         {} GB", gb).as_str()),
                    3 => output.push_str(format!("\nRAM (available):    {} GB", gb).as_str()),
                    _ => output.push_str(format!("\nRAM (unknown):      {} GB", gb).as_str()),
                }
            }
            _ => {}
        }
    }

    output
}

pub mod byte {

    pub const DATAGRAM_SIZE: usize = 32;

    pub const RESPONSE_SIZE: usize = 256;

    // f/102
    pub const FIND: u8 = 102;

    // i/105
    pub const INFO: u8 = 105;

    // h/104
    pub const HOSTNAME: u8 = 104;

    // l/108
    pub const LOAD_AVG: u8 = 108;

    // m/109
    pub const MEMINFO: u8 = 109;

    // u/117
    pub const USERNAME: u8 = 117;
}