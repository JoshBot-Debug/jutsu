use jutsu_core::segment::{self, byte};
use std::net;

const CLIENT_ADDRESS: &str = "0.0.0.0:34254";

fn main() -> std::io::Result<()> {
    {
        let socket = match net::UdpSocket::bind(CLIENT_ADDRESS) {
            Ok(v) => v,
            Err(_) => error(format!("Failed to bind socket on {CLIENT_ADDRESS}").as_str())
        };

        loop {
            let mut buf = vec![0; byte::DATAGRAM_SIZE];

            match socket.recv_from(&mut buf) {
                Ok((_, from)) => 
                {
                    dbg!(&buf);
                    dbg!(from);
                    
                    let find = segment::Find::result_from_buf(&buf);
                    dbg!(&find);

                    if let Some(username) = find
                    {
                        if let Some((meminfo, loadavg, hostname)) = segment::Info::result_from_buf(&buf)
                        {
                            dbg!(&username);
                            dbg!(&meminfo);
                            dbg!(&loadavg);
                            dbg!(&hostname);

                            let mut username = username.buf();
                            let mut meminfo = meminfo.buf();
                            let mut loadavg = loadavg.buf();
                            let mut hostname = hostname.buf();

                            let mut r_buf = Vec::with_capacity(username.len()+meminfo.len()+loadavg.len()+hostname.len());

                            r_buf.append(&mut username);
                            r_buf.append(&mut hostname);
                            r_buf.append(&mut meminfo);
                            r_buf.append(&mut loadavg);

                            dbg!(&r_buf);

                            match socket.send_to(&r_buf, from) {
                                Ok(size) => println!("Sent reply ({} bytes) to {}", size, from),
                                Err(_) => println!("Failed to send reply")
                            };
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