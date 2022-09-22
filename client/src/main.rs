use jutsu_core::ThreadPool;
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

        let mut buf = [0; 16];

        let pool = ThreadPool::new(2);

        loop {
            let message = match socket.recv_from(&mut buf) {
                Ok(v) => v,
                Err(_) =>
                {
                    eprintln!("Failed to receive packet.");
                    process::exit(1)
                }
            };
    
            pool.execute(move || {
                println!("{:?}", buf);
                dbg!(message);
            });
        }
    }
}