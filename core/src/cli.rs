#![allow(unused)]

use std::net::Ipv4Addr;

use clap::{Parser, ArgMatches};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct Ipv4AddrRange(Vec<Ipv4Addr>);

impl Ipv4AddrRange {
    pub fn foreach<T>(&self, f: T)
    where
        T: FnOnce(&Ipv4Addr) + Copy
    {
        for ip_address in &self.0 { f(ip_address) }
    }

    pub fn len(&self) -> usize { self.0.len() }

    pub fn empty() -> Self
    {
        Ipv4AddrRange(vec![])
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct SSHTarget(String, Ipv4AddrRange);

impl SSHTarget {
    pub fn targets(&self) -> (&String, &Vec<Ipv4Addr>) { (&self.0, &self.1.0) }

    pub fn len(&self) -> usize { self.0.len() }

    pub fn empty() -> Self
    {
        SSHTarget("".to_string(), Ipv4AddrRange(vec![]))
    }
}

/// Jutsu, a fast, simple and powerful tool used to find and filter linux machines
/// Copyright (C) 2022 by Joshua Joseph Myers <joshuajosephmyers@outlook.com>        
#[derive(Clone)]
#[derive(Serialize, Deserialize)]
#[derive(Parser, Debug)]
#[command(version, name = "jutsu", verbatim_doc_comment)]
pub struct Args {
    /// Target client's ipv4 address.
    /// jutsu -i 192.168.1.1		 [Single client]
    /// jutsu -i 192.168.1.1-254	 [Range of clients]
    /// jutsu -i 192.168.1.2,4,8	 [Specific clients]
    #[arg(short, long, value_parser = parse_ipv4, value_name = "ipv4 address/range", verbatim_doc_comment)]
    pub ip_address: Option<Ipv4AddrRange>,

    /// Find a client by session username.
    #[arg(short, long, value_name = "username")]
    pub username: Option<String>,

    /// Find a client by hostname.
    #[arg(long, value_name = "hostname")]
    pub hostname: Option<String>,

    /// Deploy a client via SSH.
    /// jutsu --deploy-client administrator@192.168.1.1-255      [Can be a single, range or specific clients.]
    #[arg(long, value_name = "user@host_ip", value_parser = parse_ssh_target, verbatim_doc_comment)]
    pub deploy_client: Option<SSHTarget>,

    /// Remove a client via SSH.
    /// jutsu --purge-client administrator@192.168.1.1-255       [Can be a single, range or specific clients.]
    #[arg(long, value_name = "user@host_ip", value_parser = parse_ssh_target, verbatim_doc_comment)]
    pub purge_client: Option<SSHTarget>,

    ///  Time in seconds to wait for client response.
    #[arg(short, long, default_value_t = 3, value_name = "seconds")]
    pub timeout: u32
}

impl Args {
    pub fn buf(&self) -> Result<Vec<u8>, String>
    {
        let mut packet = self.clone();
        packet.ip_address = None;
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

fn parse_ssh_target(target: &str) -> Result<SSHTarget, String> {

    let target = target.split("@").collect::<Vec<&str>>();

    let username = match target.get(0) {
        Some(u) => u.to_string(),
        None => return Err(String::from("Failed to parse username."))
    };

    let ip_addresses = match target.get(1) {
        Some(ip) =>
        {
            match parse_ipv4(ip) {
                Ok(addresses) => addresses,
                Err(e) => return Err(e)
            }
        },
        None => return Err(String::from("Failed to parse ip addresses."))
    };

    Ok(SSHTarget(username, ip_addresses))
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
        Err(_) => return Err(format!("\n{ip_string} is not a valid ipv4 address."))
    };

    for (i, s) in range.iter().enumerate()
    {
        if i == 0 { continue; }
        match s.parse::<u8>() {
            Ok(v) => targets.push(Ipv4Addr::new(first[0], first[1], first[2], v)),
            Err(_) => return Err(format!("\nThe ipv4 address ({ip_string}) provided is invalid."))
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
        Err(_) => return Err(format!("\nThe ipv4 address ({ip_string}) provided is invalid."))
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

    return Err(format!("The 6 ipv4 address ({ip_string}) provided is invalid."))
}