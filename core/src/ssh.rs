use std::{net::{TcpStream, Ipv4Addr}, io::Write, path::Path};
use ssh2::Session;
use rpassword::read_password;
use std::io::prelude::*;

use crate::cli::SSHTarget;

const CLIENT_PATH: &str = "target/x86_64-unknown-linux-musl/release/jutsu-client";
const SERVICE_PATH: &str = "jutsu-client.service";

pub fn deploy(ssh_target: SSHTarget) -> Result<(), String>
{
    print!("Password: ");
    
    if let Err(_) = std::io::stdout().flush()
    {
        return Err(String::from("Failed to flush"));
    }

    let password = std::sync::Arc::new(read_password().unwrap());

    let client_buf = match get_buf(CLIENT_PATH) {
        Ok(buf) => std::sync::Arc::new(buf),
        Err(e) =>
        {
            eprintln!("[jutsu-client] {e}");
            std::process::exit(1)
        }
    };

    let service_buf = match get_buf(SERVICE_PATH) {
        Ok(buf) => std::sync::Arc::new(buf),
        Err(e) =>
        {
            eprintln!("[jutsu-client.service] {e}");
            std::process::exit(1)
        }
    };

    let username = std::sync::Arc::new(ssh_target.targets().0.clone());

    let mut handlers = vec![];

    for ip_address in ssh_target.targets().1 {
        let ip_address = ip_address.clone();
        let password = password.clone();
        let username = username.clone();
        let client_buf = client_buf.clone();
        let service_buf = service_buf.clone();

        let join = std::thread::spawn(move || {
            let session = ssh_session(&username, &password, &ip_address);
        
            if let Err(e) = send_file(&session, &client_buf, 0o755, "jutsu-client") {
                eprintln!("[CLIENT FILE] {e} to {username}@{ip_address}");
                return;
            }
    
            if let Err(e) = send_file(&session, &service_buf, 0o755, "jutsu-client.service") {
                eprintln!("[SERVICE FILE] {e} to {username}@{ip_address}");
                return;
            }
    
            if let Err(e) = exec(&session, format!("echo {password} | sudo -S mv jutsu-client.service /etc/systemd/system/").as_str()) {
                eprintln!("[jutsu-client.service] {e}");
                return;
            }
    
            if let Err(e) = exec(&session, format!("echo {password} | sudo -S chown root:root /etc/systemd/system/jutsu-client.service").as_str()) {
                eprintln!("[jutsu-client.service] {e}");
                return;
            }
    
            if let Err(e) = exec(&session, format!("echo {password} | sudo -S ln -s /etc/systemd/system/jutsu-client.service /etc/systemd/system/multi-user.target.wants/jutsu-client.service").as_str()) {
                eprintln!("[jutsu-client.service] {e}");
                return;
            }
    
            if let Err(e) = exec(&session, format!("echo {password} | sudo -S mv jutsu-client /usr/bin/").as_str()) {
                eprintln!("[jutsu-client] {e}");
                return;
            }
    
            if let Err(e) = exec(&session, format!("echo {password} | sudo -S chown root:root  /usr/bin/jutsu-client").as_str()) {
                eprintln!("[jutsu-client] {e}");
                return;
            }
    
            if let Err(e) = exec(&session, format!("echo {password} | sudo -S systemctl daemon-reload").as_str()) {
                eprintln!("[reload systemctl] {e}");
                return;
            }
            
            if let Err(e) = exec(&session, format!("echo {password} | sudo -S systemctl restart jutsu-client").as_str()) {
                eprintln!("[start systemctl] {e}");
                return;
            }
        });
        
        handlers.push(join);
    }

    for join in handlers
    {
        join.join().unwrap()
    }

    Ok(())
}

pub fn purge(ssh_target: SSHTarget) -> Result<(), String>
{
    print!("Password: ");
    
    if let Err(_) = std::io::stdout().flush()
    {
        return Err(String::from("Failed to flush"));
    }

    let password = read_password().unwrap();

    let username = std::sync::Arc::new(ssh_target.targets().0.clone());

    let mut handlers = vec![];

    for ip_address in ssh_target.targets().1 {
        let ip_address = ip_address.clone();
        let password = password.clone();
        let username = username.clone();

        let join = std::thread::spawn(move || {
            let session = ssh_session(&username, &password, &ip_address);
        
            if let Err(e) = exec(&session, format!("echo {password} | sudo -S systemctl stop jutsu-client").as_str()) {
                eprintln!("[stop systemctl] {e}");
                return;
            }
    
            if let Err(e) = exec(&session, format!("echo {password} | sudo -S rm /etc/systemd/system/jutsu-client.service").as_str()) {
                eprintln!("[remove jutsu-client.service] {e}");
                return;
            }
    
            if let Err(e) = exec(&session, format!("echo {password} | sudo -S rm /etc/systemd/system/multi-user.target.wants/jutsu-client.service").as_str()) {
                eprintln!("[remove symlink jutsu-client.service] {e}");
                return;
            }
    
            if let Err(e) = exec(&session, format!("echo {password} | sudo -S rm /usr/bin/jutsu-client").as_str()) {
                eprintln!("[jutsu-client] {e}");
                return;
            }
    
            if let Err(e) = exec(&session, format!("echo {password} | sudo -S systemctl daemon-reload").as_str()) {
                eprintln!("[reload systemctl] {e}");
                return;
            }
        });

        handlers.push(join);
    }

    for join in handlers
    {
        join.join().unwrap()
    }

    Ok(())
}

fn exec(session: &Session, command: &str) -> Result<String, String>
{
    let mut channel = match session.channel_session() {
        Ok(c) => c,
        Err(_) => return Err(String::from("Failed to open ssh channel"))
    };
    
    if let Err(_) = channel.exec(command) { return Err(String::from("Failed to execute command over ssh channel")) }

    let mut stout = String::new();
    
    if let Err(_) = channel.read_to_string(&mut stout) { return Err(String::from("Failed to read stout from ssh channel")) }
    
    if let Err(_) = channel.wait_close() { return Err(String::from("Failed to close ssh channel")) }
    
    Ok(stout)
}


fn get_buf(path: &str) -> Result<Vec::<u8>, String>
{
    let mut buf = Vec::new();

    let mut file = match std::fs::File::open(path) {
        Ok(f) => f,
        Err(_) => return Err(String::from("Failed to find file"))
    };

    if let Err(_) = file.read_to_end(&mut buf)
    {
        return Err(String::from("Failed to read file"))
    };

    Ok(buf)
}

fn ssh_session(username: &String, password: &String, ip_address: &Ipv4Addr) -> Session
{
    let tcp = TcpStream::connect(format!("{}:22", ip_address.to_string())).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    match sess.userauth_password(username, password) {
        Err(_) =>
        {
            eprintln!("Failed to authenticate on {username}@{ip_address}");
            std::process::exit(1)
        },
        Ok(_) => {},
    };

    sess
}

fn send_file(session: &Session, buf: &Vec<u8>, permission: i32, name: &str) -> Result<(), String>
{
    let service_file = session.scp_send(Path::new(name), permission, buf.len() as u64, None);

    match service_file {
        Ok(mut channel) =>
        {
            let chunks = buf.chunks(1024);

            for chunk in chunks
            {
                channel.write(chunk).unwrap();
            }
            channel.send_eof().unwrap();
            channel.wait_eof().unwrap();
            channel.close().unwrap();
            channel.wait_close().unwrap();

            return Ok(());
        },
        Err(_) => return Err(String::from("Failed to send file"))
    }
}