use std::{net, sync::{Arc, Mutex}};
use std::io::Write;

use jutsu_cli::{Cli, CommandType};
use jutsu_core::{Datagram, segment::{self, byte}};

const DAEMON_ADDRESS: &str = "0.0.0.0:21298";
const CLIENT_PORT: u16 = 34254;

fn main() -> std::io::Result<()> {  
    {
        let socket = match net::UdpSocket::bind(DAEMON_ADDRESS) {
            Ok(v) => v,
            Err(_) => error(format!("Failed to bind socket on {DAEMON_ADDRESS}").as_str()),
        };

        let cli = Cli::new();

        let mut datagram = Datagram::new();

        let timeout = Arc::new(Mutex::new(5));

        let c_timeout = Arc::clone(&timeout);

        let arg_ip_address: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));

        cli.commands()
            .iter()
            .for_each(|command| match command.to_type() {
                CommandType::Find => {
                    if let Some(find) = command.value() {
                        datagram.push(segment::Find::new(find));
                        println!("Find {:?}", find);
                    }
                }
                CommandType::IpAddress => {
                    let mut ip_address = arg_ip_address.lock().unwrap();
                    *ip_address = command.value().clone();
                    println!("IpAddress {:?}",  command.value());
                }
                CommandType::Timeout => {
                    if let Some(time) = command.value() {
                        if let Ok(t) = time.parse::<usize>()
                        {
                            let mut ct = c_timeout.lock().unwrap();
                            *ct = t;
                            println!("Timeout {:?}", timeout);
                        }
                    }
                }
                CommandType::Info => {
                    datagram.push(segment::Info::new());
                    println!("Info {:?}", command.value());
                }
                CommandType::Help => Cli::show_help(),
                _ => {}
            });

        let buf = datagram.buf();

        cli.targets().iter().enumerate().for_each(|(_, to)| {
            let sent = socket.send_to(&buf, net::SocketAddrV4::new(*to, CLIENT_PORT));

            if let Ok(size) = sent {
                dbg!(&to, size);
            }
        });
        
        let packets_received = Arc::new(Mutex::new(0));
        let bytes_received = Arc::new(Mutex::new(0));

        let packets_received_clone1 = Arc::clone(&bytes_received);
        let bytes_received_clone1 = Arc::clone(&packets_received);

        let packets_received_clone2 = Arc::clone(&bytes_received);
        let bytes_received_clone2 = Arc::clone(&packets_received);

        let packets_received_clone3 = Arc::clone(&bytes_received);
        let bytes_received_clone3 = Arc::clone(&packets_received);


        let targets = Arc::new(Mutex::new(cli.targets().len()));

        let targets_1 =  Arc::clone(&targets);
        let targets_2 =  Arc::clone(&targets);

        let ip_address_1 =  Arc::clone(&arg_ip_address);
        let ip_address_2 =  Arc::clone(&arg_ip_address);

        ctrlc::set_handler(move || {

            let pr = packets_received_clone1.lock().unwrap();
            let br = bytes_received_clone1.lock().unwrap();
              
            statistics(*pr, *br, &ip_address_1.lock().unwrap(), &targets_1.lock().unwrap())
        })
        .expect("Error setting Ctrl-C handler");

        let mut stdout = std::io::stdout();

        std::thread::spawn(move || {
            loop {
                std::thread::sleep(std::time::Duration::from_secs(1));

                let mut ct = c_timeout.lock().unwrap();
                *ct -= 1;
                print!(".");
                stdout.flush().unwrap();
                
                if *ct < 1
                {
                    let pr = packets_received_clone2.lock().unwrap();
                    let br = bytes_received_clone2.lock().unwrap();
                    statistics(*pr, *br, &ip_address_2.lock().unwrap(), &targets_2.lock().unwrap())
                }
            }
        });

        loop {
            let mut buf = vec![0; byte::RESPONSE_SIZE];

            match socket.recv_from(&mut buf) {
                Ok((size, from)) =>
                {
                    let mut pr = packets_received_clone3.lock().unwrap();
                    let mut br = bytes_received_clone3.lock().unwrap();
                    *pr += 1;
                    *br += size;

                    let r = segment::output_from_buf(&mut buf);
                    println!("\nReceived reply ({} bytes) from {}", size, from.ip());
                    println!("{}", r);
                    println!("");
                }
                Err(_) => error("Failed to receive packet.")
            }
        }
    }

    Ok(())
}

fn statistics(packets_received: usize, bytes_received: usize, arg_ip_address: &Option<String>, targets: &usize)
{
    if let Some(ip_arg) = &arg_ip_address
    {
        println!("\r--- {} jutsu statistics ---", ip_arg);
    }
    println!("{} packets transmitted, {:?} packets ({:?} bytes) received ", &targets, &packets_received, &bytes_received);
    std::process::exit(0);
}

fn error(message: &str) -> ! {
    eprintln!("{message}");
    std::process::exit(1)
}