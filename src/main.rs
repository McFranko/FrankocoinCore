#![allow(non_snake_case)]
extern crate ed25519_dalek;
extern crate rand;
mod frankolang;
mod server;
mod test;

use std::io::Write;
use std::io::Read;

fn main() {
    // Need to add a config file

    let server = server::TcpServer {
        ipAddress: String::from("localhost:8888"), // Need to change IP to the config files IP
        handler: connectionHandler
    };
    std::thread::spawn(move || {
        server.start(50);
    });


    test::message();

    loop {}
}

fn connectionHandler(mut socket: std::net::TcpStream) {
    let mut request: [u8; 1048576] = [0; 1048576];
    socket.read(&mut request).unwrap();

    match splitBufferAt(&request, 0x0a, 1)[0] { // 0x0a is ASCII newline or \n
        b"newBlock" => {
          
        },
        &_ => return
    }


}

pub fn splitBufferAt(buffer: &[u8], pattern: u8, iterations: usize) -> Vec<&[u8]> {
    let splitBuffer = buffer.splitn(iterations, |num| *num == pattern).collect();
    return splitBuffer;
}

