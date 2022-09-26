use jutsu_core::DATAGRAM_SIZE;
use std::net;

const CLIENT_PORT: &str = "0.0.0.0:34254";

fn main() -> std::io::Result<()> {
    {
        let socket = match net::UdpSocket::bind(CLIENT_PORT) {
            Ok(v) => v,
            Err(_) => error(format!("Failed to bind socket on {CLIENT_PORT}").as_str())
        };

        loop {
            let mut buf = [0; DATAGRAM_SIZE];

            match socket.recv_from(&mut buf) {
                Ok((size, from)) => 
                {
                    println!("{size} {from} {:?}", buf);
                },
                Err(_) => error("Failed to receive packet.")
            }
        }
    }
}

fn error(message: &str) -> ! {
    eprintln!("{message}");
    std::process::exit(1)
}