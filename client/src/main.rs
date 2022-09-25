use jutsu_core::{ThreadPool, DATAGRAM_CHUNK};
use std::net;

const CLIENT_PORT: &str = "0.0.0.0:34254";

fn main() -> std::io::Result<()> {
    {
        let socket = match net::UdpSocket::bind(CLIENT_PORT) {
            Ok(v) => v,
            Err(_) => error(format!("Failed to bind socket on {CLIENT_PORT}").as_str())
        };

        let pool = ThreadPool::new(32);

        loop {
            let mut chunk = [0; DATAGRAM_CHUNK];

            match socket.recv_from(&mut chunk) {
                Ok((size, from)) => 
                {
                    pool.execute(move || { println!("{size} {from} {:?}", chunk) });
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