use clap::Parser;
use jutsu_core::cli::Ipv4AddrRange;
use jutsu_core::ssh::deploy;
use jutsu_core::{socket, cli, response::Response};
use mpsc::Receiver;
use std::io::{Write, Stdout};
use std::sync::{mpsc, Arc, Mutex};

fn main() {
    let cli = cli::Args::parse();
    
    if cli.ip_address.is_none() && cli.deploy_client.is_none() && cli.purge_client.is_none()
    {
        println!("Looks like you need some help\nCommand:\n         jutsu --help");
        std::process::exit(0)
    }

    match cli.deploy_client {
        None => {},
        Some(_) =>
        {
            deploy();
            std::process::exit(0)
        }
    }

    let socket = match socket::JutsuSocket::new(socket::SERVER_PORT) {
        Ok(socket) => socket,
        Err(e) =>
        {
            eprintln!("{e}");
            std::process::exit(1)
        }
    };

    let args = cli.buf();

    let mut stdout = std::io::stdout();

    let (exit_pkts_tx, exit_pkts_rx) = mpsc::channel::<usize>();
    let (exit_byts_tx, exit_byts_rx) = mpsc::channel::<usize>();

    let (timeout_pkts_tx, timeout_pkts_rx) = mpsc::channel::<usize>();
    let (timeout_byts_tx, timeout_byts_rx) = mpsc::channel::<usize>();
    
    let transmitted = Arc::new(Mutex::new(cli.ip_address.clone().unwrap_or(Ipv4AddrRange::empty()).len()));

    let exit_transmitted = transmitted.clone();
    let timeout_transmitted = transmitted.clone();

    let mut packets_received = 0;
    let mut bytes_received = 0;

    match args {
        Ok(buf) =>
        {
            if let Some(addresses) = cli.ip_address
            {
                addresses.foreach(|ip_address| {
                    socket.send(&buf, ip_address);
                });
            }
        },
        Err(e) =>
        {
            eprintln!("{e}");
            std::process::exit(1)
        }
    }

    match ctrlc::set_handler(move || { statistics(&exit_pkts_rx, &exit_byts_rx, &*exit_transmitted.lock().unwrap()) }) {
        Ok(_) => {},
        Err(_) =>
        {
            eprintln!("Error setting Ctrl-C handler");
            std::process::exit(1)
        }  
    }

    std::thread::spawn(move || {

        let mut timeout = cli.timeout;
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));

            if timeup(&mut stdout, &mut timeout)
            {
                statistics(&timeout_pkts_rx, &timeout_byts_rx, &*timeout_transmitted.lock().unwrap())
            }
        }
    });

    loop {
        let mut buf: Vec<u8> = vec![0; 256];
    
        match socket.receive(&mut buf) {
            Ok((response_buf, from)) =>
            {
                packets_received += 1;
                bytes_received += response_buf.len();
                let res = Response::from_buf(response_buf);
    
                match res {
                    Ok(res) =>
                    {
                        println!();
                        println!("Received reply ({} bytes) from {}", response_buf.len(), from);
                        res.print_session();
                        res.print_hostname();
                        println!();
                    },
                    Err(e) =>
                    {
                        eprintln!("{e}");
                        std::process::exit(1)
                    }
                }
                exit_pkts_tx.send(packets_received).unwrap();
                exit_byts_tx.send(bytes_received).unwrap();

                timeout_pkts_tx.send(packets_received).unwrap();
                timeout_byts_tx.send(bytes_received).unwrap();
            },
            Err(e) =>
            {
                eprintln!("{e}");
                std::process::exit(1)
            }
        }
    }
}


fn statistics(pkt_recv_rx: &Receiver<usize>, byts_recv_rx: &Receiver<usize>, sent: &usize)
{
    let received = pkt_recv_rx.try_recv().unwrap_or(0);
    let bytes_received = byts_recv_rx.try_recv().unwrap_or(0);

    println!("\r--- jutsu statistics ---\r");
    println!("{:?} packets transmitted, {:?} packets ({:?} bytes) received ", sent, received, bytes_received);
    std::process::exit(0)
}

fn timeup(stdout: &mut Stdout, timeout: &mut u32) -> bool
{
    if *timeout > 1 {
        *timeout -= 1;
        print!("\rWaiting... {:?}s", timeout);
        stdout.flush().unwrap();
        return  false;
    }
    return  true;
}