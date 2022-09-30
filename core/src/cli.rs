#![allow(unused)]

use std::net::Ipv4Addr;

use clap::{Parser, ArgMatches};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct Ipv4AddrRange(Option<Vec<Ipv4Addr>>);

impl Ipv4AddrRange {
    pub fn foreach<T>(&self, f: T)
    where
        T: FnOnce(&Ipv4Addr) + Copy
    {
        if let Some(addresses) = &self.0
        {
            for ip_address in addresses { f(ip_address) }
        }
    }

    pub fn len(&self) -> usize
    {
        match &self.0  {
            Some(v) => v.len(),
            None => 0
        }
    }
}

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
#[derive(Parser, Debug)]
#[command(version, about, author)]
pub struct Args {
   /// Target client's ipv4 address. [required]
   #[arg(short, long, value_parser = parse_ipv4, value_name = "ip_address/range")]
   pub ip_address: Ipv4AddrRange,

   /// Find a client by session username.
   #[arg(short, long)]
   pub username: Option<String>,

   ///  Time in seconds to wait for client response.
   #[arg(short, long, default_value_t = 5)]
   pub timeout: u32
}

impl Args {
    pub fn buf(&self) -> Result<Vec<u8>, String>
    {
        let mut packet = self.clone();
        packet.ip_address = Ipv4AddrRange(None);
        match bincode::serialize(&packet) {
            Ok(bytes) => Ok(bytes),
            Err(_) => Err(String::from("Failed to serialize cli."))
        }
    }

    pub fn from_buf(buf: &[u8]) -> Result<Self, String>
    {
        match bincode::deserialize(buf) {
            Ok(bytes) => Ok(bytes),
            Err(_) => Err(String::from("Failed to deserialize cli."))
        }
    }
}

fn parse_ipv4(ip_string: &str) -> Result<Ipv4AddrRange, String> {
    let mut targets = Vec::with_capacity(1020);

    let c = match ip_string.parse::<Ipv4Addr>() {
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
            if ip_string.contains("-")
            {
                if let Err(e) = range_seperated(&ip_string, &mut targets)
                {
                    return Err(e);
                }
            }
        }
    };
    
    Ok(Ipv4AddrRange(Some(targets)))
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
        return Ok(());
    }

    return Err(format!("The 6 ipv4 address ({ip_string}) provided is invalid. Use one of the patterns below."))
}