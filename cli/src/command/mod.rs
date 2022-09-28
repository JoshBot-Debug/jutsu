mod types;

use std::{env::Args, process};

pub use types::CommandType;

const HELP: &str = r#"Still not sure what this package will do! :P

Usage:
    jutsu -i <ipv4_address> [OPTIONS]...
    eg: jutsu -i 192.168.1.10 --info

Options:
    -i, --ip-address      Target client's ipv4 address. [REQUIRED]
    -t, --timeout         Time in seconds to wait for client response. Default 5s.
    -f, --find            Find a client by session username.
    --info                Get client system info.

Copyrights © 2022 joshuajosephmyers.com. All rights reserved."#;

const IP_EXAMPLE: &str = r#"

Example:
    jutsu -i 192.168.1.10        [one client]
    jutsu -i 192.168.1.50-100    [many client(s)]
    jutsu -i 192.168.1.5,10,15   [selected client(s)]
"#;

const DUPLICATE_IP: &str = r#"Duplicate ip address (-i, --ip-address) commands found. Did you mean to pass an ip range?

Example:
    jutsu -i 192.168.1.10-50   [many client(s)]
"#;

#[derive(Debug)]
pub struct Command
{
    arg: String,
    value: Option<String>
}

impl Command {
    pub fn new(arg: String, value: Option<String>) -> Self {
        Self {arg, value}
    }

    pub fn from_args(args: Args) -> Vec<Command> {
        let mut result: Vec<Command> = Vec::new();

        let args = args.collect::<Vec<String>>();

        if !(args.contains(&"-i".to_string()) || args.contains(&"--ip-address".to_string())) { show_help() }

        for (index, argument) in args.iter().enumerate() {
            match self::from_arg(&args, argument, index) {
                Some(command) => result.push(command),
                None => continue,
            }
        }
        
        result
    }

    pub fn value(&self) -> &Option<String>
    {
        &self.value
    }

    pub fn to_type(&self) -> CommandType {
        match self.arg.as_str() {
            "--find" => CommandType::Find,
            "--ip-address" => CommandType::IpAddress,
            "--info" => CommandType::Info,
            "--help" => CommandType::Help,
            "--timeout" => CommandType::Timeout,
            _ => CommandType::None,
        }
    }
}


pub fn show_help() -> !
{
    println!("{HELP}");
    process::exit(0)
}

pub fn show_example_ip(message: &str) -> !
{
    println!("{message}{IP_EXAMPLE}");
    process::exit(0)
}

fn from_arg(args: &Vec<String>, argument: &String, index: usize) -> Option<Command> {
    match argument.as_str() {
        "-h" | "--help" => Some(Command {arg: "--help".to_string(), value: None}),
        "-t" | "--timeout" => {
            let val = match args.get(index + 1) {
                Some(v) => v.parse::<usize>().unwrap_or_else(|_| error(format!(
                    "Timeout (-t, --timeout) requires an integer value.\nExample: -t 5"
                ))),
                None => error(format!(
                    "Timeout (-t, --timeout) requires a value.\nExample: -t 5"
                )),
            };
            Some(Command {arg: "--timeout".to_string(), value: Some(val.to_string())})
        },
        "-f" | "--find" => {
            allow_once(args, Some("-f"), Some("--find"), format!("Duplicate find (-f, --find) commands found. You cannot pass multiple find (-f, --find) commands."));
            let val = match args.get(index + 1) {
                Some(v) => is_value(argument, v),
                None => error(format!(
                    "Find (-f, --find) requires a value.\nExample: -f <username>"
                )),
            };
            Some(Command {arg: "--find".to_string(), value: Some(val.clone())})
        },
        "-i" | "--ip-address" => {
            allow_once(args, Some("-i"), Some("--ip-address"), DUPLICATE_IP.to_string());
            let val = match args.get(index + 1) {
                Some(v) => is_value(argument, v),
                None => show_example_ip("Ip address (-i, --ip-address) requires a value."),
            };
            Some(Command {arg: "--ip-address".to_string(), value: Some(val.clone())})
        },
        "--info" => Some(Command {arg: "--info".to_string(), value: None}),
        _ => {
            if argument.starts_with("-") {
                eprintln!("Invalid command {}\n", argument);
                show_help()
            }
            None
        }
    }
}

fn error(message: String) -> ! {
    eprintln!("{message}");
    process::exit(1)
}

fn is_value<'a>(argument: &String, value: &'a String) -> &'a String {
    if value.starts_with("-") {
        eprintln!("Invalid value {value} for argument {argument}");
        process::exit(1)
    }
    value
}

fn allow_once(args: &Vec<String>, short: Option<&str>, long: Option<&str>, message: String) {
    if args
        .iter()
        .filter(|&v| {
            (short.is_some() && v.eq(short.unwrap())) || long.is_some() && v.eq(long.unwrap())
        })
        .count()
        > 1
    {
        eprintln!("{message}");
        process::exit(1)
    }
}
