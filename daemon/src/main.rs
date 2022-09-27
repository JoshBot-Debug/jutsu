use std::net;

use jutsu_cli::{Cli, CommandType};
use jutsu_core::{Datagram, Find, Info, Hostname};

const DAEMON_PORT: &str = "0.0.0.0:34255";

fn main() -> std::io::Result<()> {
    {
        let socket = match net::UdpSocket::bind(DAEMON_PORT) {
            Ok(v) => v,
            Err(_) => error(format!("Failed to bind socket on {DAEMON_PORT}").as_str()),
        };

        let cli = Cli::new();

        let mut datagram = Datagram::new();

        cli.commands()
            .iter()
            .for_each(|command| match command.to_type() {
                CommandType::Find => {
                    if let Some(find) = command.value() {
                        datagram.push(Find::new(find));
                        println!("Find {:?}", find);
                    }
                }
                CommandType::IpAddress => {
                    println!("IpAddress {:?}", cli.targets());
                }
                CommandType::Info => {
                    datagram.push(Info::new());
                    println!("Info {:?}", command.value());
                }
                CommandType::Help => Cli::show_help(),
                _ => {}
            });

        let buf = datagram.buf();

        cli.targets().iter().enumerate().for_each(|(_, to)| {
            let sent = socket.send_to(&buf, net::SocketAddrV4::new(*to, 34254));

            if let Ok(size) = sent {
                dbg!(&to, size);
            }
        });

        loop {
            let mut buf = vec![0; 128];

            match socket.recv_from(&mut buf) {
                Ok((size, from)) =>
                {
                    println!("Received reply ({} bytes) from {}", size, from);
                    buf.truncate(size);

                    let find = Find::from_buf(&buf);
                    let hostname = Hostname::from_buf(&buf);
                    dbg!(&buf);
                    dbg!(&from);
                    dbg!(&find);
                    dbg!(&hostname);
                    break;
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