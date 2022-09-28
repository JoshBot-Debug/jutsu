use std::net;

use jutsu_cli::{Cli, CommandType};
use jutsu_core::{Datagram, segment::{self, byte}};

const DAEMON_ADDRESS: &str = "0.0.0.0:21298";
const CLIENT_PORT: u16 = 34254;

fn main() -> std::io::Result<()> {  
    {
        let socket = match net::UdpSocket::bind(DAEMON_ADDRESS) {
            Ok(v) => v,
            Err(_) => error(format!("Failed to bind socket on {DAEMON_ADDRESS}").as_str()),
        };

        let cli = Cli::new();

        let mut datagram = Datagram::new();

        cli.commands()
            .iter()
            .for_each(|command| match command.to_type() {
                CommandType::Find => {
                    if let Some(find) = command.value() {
                        datagram.push(segment::Find::new(find));
                        println!("Find {:?}", find);
                    }
                }
                CommandType::IpAddress => {
                    println!("IpAddress {:?}", cli.targets());
                }
                CommandType::Info => {
                    datagram.push(segment::Info::new());
                    println!("Info {:?}", command.value());
                }
                CommandType::Help => Cli::show_help(),
                _ => {}
            });

        let buf = datagram.buf();

        cli.targets().iter().enumerate().for_each(|(_, to)| {
            let sent = socket.send_to(&buf, net::SocketAddrV4::new(*to, CLIENT_PORT));

            if let Ok(size) = sent {
                dbg!(&to, size);
            }
        });

        loop {
            let mut buf = vec![0; byte::RESPONSE_SIZE];

            match socket.recv_from(&mut buf) {
                Ok((size, from)) =>
                {
                    let r = segment::output_from_buf(&mut buf);
                    println!("{}", r);
                    println!("Received reply ({} bytes) from {}", size, from);
                }
                Err(_) => error("Failed to receive packet.")
            }
        }
    }

    Ok(())
}

fn error(message: &str) -> ! {
    eprintln!("{message}");
    std::process::exit(1)
}