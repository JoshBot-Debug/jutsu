use std::net;

use jutsu_cli::{Cli,CommandType};


fn main() -> std::io::Result<()> {
    {
        let socket = match net::UdpSocket::bind("0.0.0.0:34255") {
            Ok(v) => v,
            Err(_) => error("Failed to bind socket on 0.0.0.0:34255")
        };

        let cli = Cli::new();
        
        cli
        .commands()
        .iter()
        .for_each(|command| {
            match command.to_type() {
                CommandType::Find =>
                {
                    println!("Find user {:?} : IP {:?}", command.value(), cli.targets());
                },
                CommandType::Info => 
                {
                    println!("Info");
                },
                CommandType::Help => Cli::show_help(),
                _ => {}
            }
        });

        // cli
        // .targets()
        // .iter()
        // .enumerate()
        // .for_each(|(index, ipv4)| {
            
        //     let buf: [u8; 16] = [index.try_into().unwrap(); 16];

        //     let _sent = socket.send_to(&buf, net::SocketAddrV4::new(*ipv4, 34254));
            
        //     // if let Ok(size) = sent
        //     // {
        //     //     // println!("{size}")
        //     // }
        // });



        // dbg!(&cli.execute);


        // let execute = match Execute::from_str(&cli.execute.unwrap()) {
        //     Ok(e) => e,
        //     Err(e) =>
        //     {
        //         eprintln!("{e}");
        //         process::exit(e.code);
        //     }
        // };

        // let payload = Packet16::new(execute);

        // dbg!(&payload);

        // let buf = &mut [0; 16];

        // let (packet_size, from) = match socket.recv_from(buf) {
        //     Ok(v) => v,
        //     Err(_) =>
        //     {
        //         eprintln!("Failed to receive packet.");
        //         process::exit(1)
        //     }
        // };

        // dbg!(&packet_size);
        // dbg!(&from);

        // let packet = Packet16::from_buf(buf);

        // dbg!(&packet);
    }
    
    Ok(())
}

fn error(message: &str) -> ! {
    eprintln!("{message}");
    std::process::exit(1)
}