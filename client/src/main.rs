use jutsu_core::{socket::{JutsuSocket, CLIENT_PORT}, cli};

fn main() {
    
    let socket = match JutsuSocket::new(CLIENT_PORT) {
        Ok(socket) => socket,
        Err(e) =>
        {
            eprintln!("{e}");
            std::process::exit(1)
        }
    };

    let mut buf: Vec<u8> = vec![0; 16];
    
    match socket.receive(&mut buf) {
        Ok((packet, from)) =>
        {
            dbg!(&packet);
            dbg!(&packet);
            
            dbg!(cli::Args::from_buf(&packet.to_vec()).unwrap());
            println!("[CLIENT] Received packet from {}", from);
            socket.send(&packet,&from);
        },
        Err(e) =>
        {
            eprintln!("{e}");
            std::process::exit(1)
        }
    }
}