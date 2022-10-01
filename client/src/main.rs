use jutsu_core::{socket, response, cli};

fn main() {
    
    let socket = match socket::JutsuSocket::new(socket::CLIENT_PORT) {
        Ok(socket) => socket,
        Err(e) =>
        {
            eprintln!("{e}");
            std::process::exit(1)
        }
    };

    let mut response = response::Response::new();


    loop {
        let mut buf: Vec<u8> = vec![0; 256];
        
        match socket.receive(&mut buf) {
            Ok((cli_buf, from)) =>
            {         
                println!("Received packet from {}", from);

                response.refresh();

                let cli = cli::Args::from_buf(&cli_buf);

                dbg!(&cli);

                match cli {
                    Ok(args) =>
                    {
                        dbg!(&args);

                        if filter(&args, &response)
                        {
                            match response.buf() {
                                Ok(buf) =>
                                {
                                    socket.send(&buf.to_vec(),&from);
                                },
                                Err(e) => {
                                    eprintln!("{e}");
                                    std::process::exit(1)
                                }
                            }
                        }
                    },
                    Err(e) =>
                    {
                        eprintln!("{e}");
                        std::process::exit(1)
                    }
                }
            }
            Err(e) =>
            {
                eprintln!("{e}");
                std::process::exit(1)
            }
        }
    }
}


fn filter(args: &cli::Args, response: &response::Response) -> bool
{
    return (
        match &args.username { Some(username) => response.session_includes(&username), None => false} ||
        match &args.hostname { Some(hostname) => response.hostname_includes(&hostname), None => false }
    )
}