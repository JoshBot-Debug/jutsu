use jutsu_core::{DATAGRAM_SIZE, Find, Info, MemInfo};
use std::net;

const CLIENT_PORT: &str = "0.0.0.0:34254";

fn main() -> std::io::Result<()> {
    {
        let socket = match net::UdpSocket::bind(CLIENT_PORT) {
            Ok(v) => v,
            Err(_) => error(format!("Failed to bind socket on {CLIENT_PORT}").as_str())
        };

        MemInfo::new();

        loop {
            let mut buf = vec![0; DATAGRAM_SIZE];

            match socket.recv_from(&mut buf) {
                Ok((_, from)) => 
                {
                    // dbg!(&buf);
                    dbg!(from);
                    
                    let find = Find::result_from_buf(&buf);
                    dbg!(&find);

                    if let Some(users) = find
                    {
                        if let Some((meminfo, loadavg, hostname)) = Info::result_from_buf(&buf)
                        {
                            dbg!(&users);
                            dbg!(&meminfo);
                            dbg!(&loadavg);
                            dbg!(&hostname);
                        }
                    }
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