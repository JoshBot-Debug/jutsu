use jutsu_core::{ThreadPool, DATAGRAM_CHUNK};
use std::{net, process};


fn main() -> std::io::Result<()> {
    {
        let socket = match net::UdpSocket::bind("0.0.0.0:34254") {
            Ok(v) => v,
            Err(_) =>
            {
                eprintln!("Failed to bind socket on 0.0.0.0:34254");
                process::exit(1)
            }
        };


        let pool = ThreadPool::new(32);

        loop {
            let mut chunk = [0; DATAGRAM_CHUNK];

            let message = match socket.recv_from(&mut chunk) {
                Ok(v) => v,
                Err(_) =>
                {
                    eprintln!("Failed to receive packet.");
                    process::exit(1)
                }
            };
    
            pool.execute(move || {
                println!("{:?}", chunk);
                dbg!(message);
            });
        }
    }
}