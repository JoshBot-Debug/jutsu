use clap::Parser;
use jutsu_core::{socket, cli};

fn main() {
    let cli = cli::Args::parse();
    
    let socket = match socket::JutsuSocket::new(socket::SERVER_PORT) {
        Ok(socket) => socket,
        Err(e) =>
        {
            eprintln!("{e}");
            std::process::exit(1)
        }
    };

    match cli.buf() {
        Ok(buf) =>
        {
            cli.ip_address.foreach(|ip_address| {
                println!("{:?}", ip_address);
                socket.send(&buf, ip_address);
            });
        }
        Err(e) =>
        {
            eprintln!("{e}");
            std::process::exit(1)
        }
    }

    let mut buf: Vec<u8> = vec![0; 16];
    
    match socket.receive(&mut buf) {
        Ok((packet, from)) =>
        {
            dbg!(packet);
            println!("[DAEMON] Received packet from {}", from);
        },
        Err(e) =>
        {
            eprintln!("{e}");
            std::process::exit(1)
        }
    }
}