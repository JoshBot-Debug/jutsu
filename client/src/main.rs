use jutsu_core::{DATAGRAM_SIZE, Find};
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
                    // dbg!(&buf);
                    dbg!(from);

                    // let find = Find::from_buf(&buf).unwrap();
                    let find = Find::result_from_buf(&buf).unwrap();
                    dbg!(&find);

                    // let t = buf.get(0);

                    // if let Some(t) = t
                    // {
                    //    dbg!(t);

                    //    if let Some(ts) = buf.get(2)
                    //    {
                    //         dbg!(ts);

                    //         if let Some(td) = buf.get(3..(3+ts).into())
                    //         {
                    //             dbg!(td);
                    //             let td_as_string = String::from_utf8(td.to_vec());
                                
                    //             if let Ok(td_as_string) = td_as_string
                    //             {
                    //                 dbg!(td_as_string);
                    //             }
                    //         }
                    //    }
                    // }
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