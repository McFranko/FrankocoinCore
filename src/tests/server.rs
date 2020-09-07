
use crate::*;

#[test]
fn serverTest() {
    let server = server::TcpServer {
        ipAddress: String::from("localhost:8888"), // Need to change IP to the config files IP
        handler: connectionHandler
    };
    std::thread::spawn(move || {
        server.start(50);
    });
}

fn connectionHandler(mut socket: std::net::TcpStream)
{
    let mut request: [u8; 1048576] = [0; 1048576];

    socket.read(&mut request).unwrap();
}
