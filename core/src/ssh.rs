use std::net::TcpStream;
use ssh2::Session;


pub fn deploy()
{
    // Connect to the local SSH server
    let tcp = TcpStream::connect("127.0.0.1:22").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    // Try to authenticate with the first identity in the agent.
    sess.userauth_password("josh", "Comperswift").unwrap();

    // Make sure we succeeded
    println!("Auth: {}", sess.authenticated());
}