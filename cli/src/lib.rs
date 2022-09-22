mod command;

use std::net;

pub use command::{Command, CommandType};

#[derive(Debug, Default)]
pub struct Cli
{
    targets: Vec<net::Ipv4Addr>,
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

    pub fn targets(&self) -> &Vec<net::Ipv4Addr>
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


fn parse_target_ipv4<'a>(commands: &Vec<Command>, targets: &mut Vec<net::Ipv4Addr>)
{
    for command in commands.iter()
    {
        if CommandType::IpAddress == command.to_type()
        {
            match command.value() {
                Some(ip_string) =>
                {
                    match ip_string.parse::<net::Ipv4Addr>() {
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

fn parse_ip_comma(ip_string: &String, targets: &mut Vec<net::Ipv4Addr>)
{
    let range: Vec<&str> = ip_string.split(",").collect();

    let first = match range
    .get(0)
    .unwrap()
    .parse::<net::Ipv4Addr>() {
        Ok(v) =>
        {
            targets.push(v);
            v.octets()
        },
        Err(_) => command::show_example_ip(format!("{ip_string} is not a valid ipv4 address. Use one of the patterns below.").as_str())
    };


    for (i, s) in range.iter().enumerate()
    {
        if i == 0 { continue; }
        match s.parse::<u8>() {
            Ok(v) => targets.push(net::Ipv4Addr::new(first[0], first[1], first[2], v)),
            Err(_) => command::show_example_ip(format!("The ipv4 address ({ip_string}) provided is invalid. Use one of the patterns below.").as_str())
        }
    }
}

fn parse_ip_range(ip_string: &String, targets: &mut Vec<net::Ipv4Addr>)
{
    let range: Vec<&str> = ip_string.split("-").collect();
                                    
    let first = match range
    .get(0)
    .unwrap()
    .parse::<net::Ipv4Addr>() {
        Ok(v) => v.octets(),
        Err(_) => command::show_example_ip(format!("The ipv4 address ({ip_string}) provided is invalid. Use one of the patterns below.").as_str())
    };

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
    .unwrap_or_else(|_| command::show_example_ip(format!("The ipv4 address ({ip_string}) provided is invalid. Use one of the patterns below.").as_str()));

    for i in start..=end {
        targets.push(net::Ipv4Addr::new(first[0], first[1], first[2], i.try_into().unwrap()))
    }
}


#[cfg(test)]
mod tests {
    use std::net;
    use crate::{parse_target_ipv4, Command};


    #[test]
    fn ip_address_empty() {
        
        let mut targets: Vec<net::Ipv4Addr> = Vec::new();
        let command: Vec<Command> = vec![
            Command::new("--ip-address".to_string(), None)
        ];

        parse_target_ipv4(&command, &mut targets);
    }

    #[test]
    fn ip_address_invalid() {
        
        let mut targets: Vec<net::Ipv4Addr> = Vec::new();
        let command: Vec<Command> = vec![
            Command::new("--ip-address".to_string(), Some("192.168.1.a".to_string()))
        ];

        parse_target_ipv4(&command, &mut targets);
    }

    #[test]
    fn ip_address_invalid_comma() {
        
        let mut targets: Vec<net::Ipv4Addr> = Vec::new();
        let command: Vec<Command> = vec![
            Command::new("--ip-address".to_string(), Some("192.168.1.1,as".to_string()))
        ];

        parse_target_ipv4(&command, &mut targets);
    }

    #[test]
    fn ip_address_invalid_range() {
        
        let mut targets: Vec<net::Ipv4Addr> = Vec::new();
        let command: Vec<Command> = vec![
            Command::new("--ip-address".to_string(), Some("192.168.1.1-256".to_string()))
        ];

        parse_target_ipv4(&command, &mut targets);
    }
}