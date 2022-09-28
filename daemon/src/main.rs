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

        let mut timeout: usize = 5;

        let mut arg_ip_address: Option<String> = None;

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
                    arg_ip_address = command.value().clone();
                    println!("IpAddress {:?}",  command.value());
                }
                CommandType::Timeout => {
                    if let Some(time) = command.value() {
                        if let Ok(t) = time.parse::<usize>()
                        {
                            timeout = t;
                            println!("Timeout {:?}", timeout);
                        }
                    }
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
        
        let bytes_received = std::sync::Arc::new(std::sync::Mutex::new(0));
        let packets_received = std::sync::Arc::new(std::sync::Mutex::new(0));

        let pr = std::sync::Arc::clone(&packets_received);
        let br = std::sync::Arc::clone(&bytes_received);

        ctrlc::set_handler(move || {
            print!("\n");
            let pr = pr.lock().unwrap();
            let br = br.lock().unwrap();
            if let Some(ip_arg) = &arg_ip_address
            {
                println!("--- {} jutsu statistics ---", ip_arg);
            }
            println!("{} packets transmitted, {:?} packets ({:?} bytes) received ", &cli.targets().len(), &pr, &br);
            std::process::exit(0);
        })
        .expect("Error setting Ctrl-C handler");

        loop {
            let mut buf = vec![0; byte::RESPONSE_SIZE];
            let packets_received = std::sync::Arc::clone(&packets_received);
            let bytes_received = std::sync::Arc::clone(&bytes_received);

            match socket.recv_from(&mut buf) {
                Ok((size, from)) =>
                {
                    let mut pr = packets_received.lock().unwrap();
                    let mut br = bytes_received.lock().unwrap();
                    *pr += 1;
                    *br += size;

                    let r = segment::output_from_buf(&mut buf);
                    println!("\nReceived reply ({} bytes) from {}", size, from.ip());
                    println!("{}", r);
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