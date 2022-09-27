use jutsu_core::{DATAGRAM_SIZE, Find, Info};
use std::net;

const CLIENT_PORT: &str = "0.0.0.0:34254";

fn main() -> std::io::Result<()> {
    {
        let socket = match net::UdpSocket::bind(CLIENT_PORT) {
            Ok(v) => v,
            Err(_) => error(format!("Failed to bind socket on {CLIENT_PORT}").as_str())
        };

        loop {
            let mut buf = vec![0; DATAGRAM_SIZE];

            match socket.recv_from(&mut buf) {
                Ok((_, from)) => 
                {
                    dbg!(&buf);
                    dbg!(from);
                    
                    let find = Find::result_from_buf(&buf);
                    dbg!(&find);

                    if let Some((username, mut user)) = find
                    {
                        if let Some((meminfo, loadavg, hostname)) = Info::result_from_buf(&buf)
                        {
                            dbg!(&username);
                            dbg!(&meminfo);
                            dbg!(&loadavg);
                            dbg!(&hostname);

                            let mut meminfo = meminfo.buf();
                            let mut loadavg = loadavg.buf();
                            let mut hostname = hostname.buf();

                            let mut r_buf = Vec::with_capacity(user.len()+meminfo.len()+loadavg.len()+hostname.len());

                            r_buf.append(&mut user);
                            r_buf.append(&mut meminfo);
                            r_buf.append(&mut loadavg);
                            r_buf.append(&mut hostname);

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