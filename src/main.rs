#![allow(non_snake_case)]
extern crate md5;
extern crate ed25519_dalek;
extern crate rand;
extern crate serde;
extern crate bincode;

mod frankolang;
mod server;
mod test;

use std::io::Read;

fn main()
{
    // Need to add a config file

    let server = server::TcpServer {
        ipAddress: String::from("localhost:8888"), // Need to change IP to the config files IP
        handler: connectionHandler
    };
    let serverThread = std::thread::spawn(move || {
        server.start(50);
    });

    test::runTests();

    serverThread.join().unwrap();
}

fn connectionHandler(mut socket: std::net::TcpStream)
{
    let mut request: [u8; 1048576] = [0; 1048576];

    socket.read(&mut request).unwrap();
}
