use std::net::{UdpSocket, Ipv4Addr, SocketAddrV4, IpAddr};

pub const SERVER_PORT: u16 = 21298;
pub const CLIENT_PORT: u16 = 21299;

pub struct JutsuSocket
{
    socket: UdpSocket,
    port: u16
}

impl JutsuSocket
{
    pub fn new(port: u16) -> Result<Self, String>
    {
        let socket = match UdpSocket::bind(format!("0.0.0.0:{}", port)) {
            Ok(socket) => socket,
            Err(_) =>
            {
                return Err(format!("Failed to bind socket on 0.0.0.0:{port}"));
            },
        };

        let port = if port == SERVER_PORT { CLIENT_PORT } else { SERVER_PORT };

        Ok(Self { socket, port })
    }

    pub fn send(&self, buf: &Vec<u8>, to: &Ipv4Addr) -> bool
    {
        match self.socket.send_to(buf, SocketAddrV4::new(*to, self.port)) {
            Ok(_) => true,
            Err(_) => false
        }
    }

    pub fn receive<'b>(&self, buf: &'b mut [u8]) -> Result<(&'b [u8], Ipv4Addr), String>
    {
        match self.socket.recv_from(buf) {
            Ok((length, from)) =>
            {
                let from = match from.ip() {
                    IpAddr::V4(ip) => ip,
                    _ => return Err(String::from("Received from contains an invalid ipv4 address."))
                };
                Ok((&buf[..length], from))
            },
            Err(_) => Err(String::from("Failed to receive packet."))
        }
    }
}