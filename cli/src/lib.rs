mod command;

use std::{net::Ipv4Addr, process};

pub use command::{Command, CommandType};

#[derive(Debug, Default)]
pub struct Cli
{
    targets: Vec<Ipv4Addr>,
    commands: Vec<Command>
}

impl Cli {

    pub fn new() -> Self {
        let args = std::env::args();
        let commands = Command::from_args(args);
        let mut targets = vec![];
        
        parse_target_ipv4(&commands, &mut targets);

        Self { commands, targets }
    }

    pub fn targets(&self) -> &Vec<Ipv4Addr>
    {
        &self.targets
    }

    pub fn commands(&self) -> &Vec<Command> {
        &self.commands
    }

    pub fn show_help() -> !
    {
        command::show_help()
    }
}


fn parse_target_ipv4<'a>(commands: &Vec<Command>, targets: &mut Vec<Ipv4Addr>)
{
    for command in commands.iter()
    {
        if CommandType::IpAddress == command.to_type()
        {
            match command.value() {
                Some(ip_string) =>
                {
                    match ip_string.parse::<Ipv4Addr>() {
                        Ok(ip_address) => targets.push(ip_address),
                        Err(_) =>
                        {
                            if ip_string.contains(",")
                            {
                                parse_ip_comma(ip_string, targets);
                                return;
                            }
                            parse_ip_range(ip_string, targets);
                        }
                    }
                },
                None => command::show_help()
            }
        }
    }
}

fn parse_ip_comma(ip_string: &String, targets: &mut Vec<Ipv4Addr>)
{
    let range: Vec<&str> = ip_string.split(",").collect();

    let first = match range.get(0) {
        None => error(format!("Invalid ipv4 address.").as_str()),
        Some(v) =>
            match v.parse::<Ipv4Addr>() {
                Ok(v) =>
                {
                    targets.push(v);
                    v.octets()
                },
                Err(_) => error(format!("{v} is not a valid ipv4 address.").as_str())
            }
    };


    for (i, s) in range.iter().enumerate()
    {
        if i == 0 { continue; }
        match s.parse::<u8>() {
            Ok(v) => targets.push(Ipv4Addr::new(first[0], first[1], first[2], v)),
            Err(_) => error(format!("{ip_string} contains an invalid ipv4 addresses.").as_str())
        }
        
    }
}

fn parse_ip_range(ip_string: &String, targets: &mut Vec<Ipv4Addr>)
{
    let range: Vec<&str> = ip_string.split("-").collect();
                                    
    let first = parse_ip_string(&range);

    let start = range
    .get(0)
    .unwrap()
    .split(".")
    .last()
    .unwrap()
    .parse::<u8>()
    .unwrap();
    

    let end = range
    .get(1)
    .unwrap()
    .parse::<u8>()
    .unwrap_or_else(|_| error(format!("{ip_string} is not a valid ipv4 address range.").as_str()));

    for i in start..=end {
        targets.push(Ipv4Addr::new(first[0], first[1], first[2], i.try_into().unwrap()))
    }
}

fn parse_ip_string(range: &Vec<&str>) -> [u8; 4]
{
    match range.get(0) {
        None => error(format!("Invalid ipv4 address.").as_str()),
        Some(v) =>
            match v.parse::<Ipv4Addr>() {
                Ok(v) => v.octets(),
                Err(_) => error(format!("{v} is not a valid ip address.").as_str())
            }
    }
}

fn error(message: &str) -> ! {
    eprintln!("{message}");
    process::exit(1)
}