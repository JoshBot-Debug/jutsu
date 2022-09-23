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

            match socket.recv_from(&mut chunk) {
                Ok(_) => 
                {
                    pool.execute(move || {
                        println!("{:?}", chunk);
                    });
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
