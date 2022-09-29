#![allow(unused)]

use std::net::Ipv4Addr;

use clap::{Parser, ArgMatches};

#[derive(Debug, Clone)]
pub struct Ipv4AddrRange(Vec<Ipv4Addr>);

impl Ipv4AddrRange {
    pub fn foreach<T>(&self, f: T)
    where
        T: FnOnce(&Ipv4Addr) + Copy
    {
        for ip_address in &self.0 { f(ip_address) }
    }
}

#[derive(Debug, Clone)]
pub struct Username(Option<String>);

impl Username {

    fn byte() -> u8 {117}

    pub fn from_buf(buf: &Vec<u8>) -> Result<Self, String> {
        if let Some(i) = buf.iter().enumerate().position(|(i, r)| {
            *r == Username::byte()
                && *buf
                    .get(i + 1)
                    .unwrap_or_else(|| error("Failed to parse buffer".to_string()))
                    == 0
        }) {
            if let Ok(value) = String::from_utf8(
                buf.get(
                    i + 3
                        ..i + 3
                            + *buf
                                .get(i + 2)
                                .unwrap_or_else(|| error("Failed to parse buffer".to_string()))
                                as usize,
                )
                .unwrap_or_else(|| error("Failed to parse buffer".to_string()))
                .to_vec(),
            ) {
                return Ok(Self(Some(value)));
            }
        }

        Err(format!("Failed to get username from buffer."))
    }
}

fn error(message: String) -> ! {
    eprintln!("{message}");
    std::process::exit(1)
}

#[derive(Parser, Debug)]
#[command(version, about, author)]
pub struct Args {
   /// Target client's ipv4 address. [required]
   #[arg(short, long, value_parser = parse_ipv4, value_name = "ip_address/range")]
   pub ip_address: Ipv4AddrRange,

   /// Find a client by session username.
   #[arg(short, long, value_parser = parse_username)]
   pub username: Username,

   ///  Time in seconds to wait for client response.
   #[arg(short, long, default_value_t = 5)]
   pub timeout: u32
}

impl Args {
    pub fn buf(&self) -> Result<Vec<u8>, String>
    {
        let mut buf = Vec::with_capacity(32);

        match &self.username.0 {
            Some(username) =>
            {
                buf.append(&mut [Username::byte(), 0, username.len() as u8].to_vec());
                buf.append(&mut username.as_bytes().to_vec())
            },
            None => {}
        }

        Ok(buf)
    }

    pub fn from_buf(buf: &Vec<u8>) -> Result<Args, String>
    {
        let ip_address = Ipv4AddrRange(Vec::new());

        let username = match Username::from_buf(buf) {
            Ok(username) => username,
            Err(_) => return Err(format!("Failed to get username from buffer."))
        };

        Ok(Args { ip_address, username, timeout: 0 })
    }
}




fn parse_username(username: &str) -> Result<Username, String> {   
    Ok(Username(Some(username.to_string())))
}

fn parse_ipv4(ip_string: &str) -> Result<Ipv4AddrRange, String> {
    let mut targets = Vec::with_capacity(1020);

    match ip_string.parse::<Ipv4Addr>() {
        Ok(ip_address) => targets.push(ip_address),
        Err(_) =>
        {
            if ip_string.contains(",")
            {
                if let Err(e) = comma_seperated(&ip_string, &mut targets)
                {
                    return Err(e);
                }
            }
            if let Err(e) = range_seperated(&ip_string, &mut targets)
            {
                return Err(e);
            }
        }
    }
    
    Ok(Ipv4AddrRange(targets))
}

fn comma_seperated(ip_string: &str, targets: &mut Vec<Ipv4Addr>) -> Result<(), String>
{
    let range: Vec<&str> = ip_string.split(",").collect();

    let first = match range
    .get(0)
    .unwrap()
    .parse::<Ipv4Addr>() {
        Ok(v) =>
        {
            targets.push(v);
            v.octets()
        },
        Err(_) => return Err(format!("\n{ip_string} is not a valid ipv4 address. Use one of the patterns below."))
    };

    for (i, s) in range.iter().enumerate()
    {
        if i == 0 { continue; }
        match s.parse::<u8>() {
            Ok(v) => targets.push(Ipv4Addr::new(first[0], first[1], first[2], v)),
            Err(_) => return Err(format!("\nThe ipv4 address ({ip_string}) provided is invalid. Use one of the patterns below."))
        }
    }

    Ok(())
}

fn range_seperated(ip_string: &str, targets: &mut Vec<Ipv4Addr>) -> Result<(), String>
{
    let range: Vec<&str> = ip_string.split("-").collect();
                                    
    let first = match range
    .get(0)
    .unwrap()
    .parse::<Ipv4Addr>() {
        Ok(v) => v.octets(),
        Err(_) => return Err(format!("\nThe ipv4 address ({ip_string}) provided is invalid. Use one of the patterns below."))
    };

    let start = range
    .get(0)
    .unwrap()
    .split(".")
    .last()
    .unwrap()
    .parse::<u8>()
    .unwrap();

    if let Ok(end) = range.get(1).unwrap().parse::<u8>()
    {
        for i in start..=end {
            targets.push(Ipv4Addr::new(first[0], first[1], first[2], i.try_into().unwrap()))
        }
    }

    return Err(format!("The ipv4 address ({ip_string}) provided is invalid. Use one of the patterns below."))
}