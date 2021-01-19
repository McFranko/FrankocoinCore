use crate::server::TcpServer;

use std::{
    error::Error,
    time::Duration,
    io::{Read, Write},
    thread,
    net::{TcpStream, Shutdown},
};

#[test]
fn server() -> Result<(), Box<dyn Error>> {
    let server = TcpServer::new(
        String::from("localhost:8888"), // Need to change IP to the config files IP
        connectionHandler,
    );
    thread::spawn(move || {
        server.start(50).unwrap();
    });

    // Gives the server time to startup
    thread::sleep(Duration::from_millis(10));

    let mut stream = TcpStream::connect("localhost:8888")?;
    stream.write(b"Hello World!")?;
    stream.shutdown(Shutdown::Both)?;

    Ok(())
}

fn connectionHandler(mut socket: TcpStream) {
    let mut request: Vec<u8> = Vec::new();

    socket.read_to_end(&mut request).unwrap();
    assert_eq!(&request, b"Hello World!")
}
