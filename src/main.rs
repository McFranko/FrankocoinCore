#![allow(non_snake_case)]
extern crate ed25519_dalek;
extern crate rand;
mod frankolang;
mod server;

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

    let mut message = [0u8; 180];
    generateTestTransaction(&mut message);


    let codeIsGood = frankolang::interpretFrankolang(&message, true);

    if codeIsGood {
        println!("Valid frankolang");
        frankolang::interpretFrankolang(&message, false);
    } else {
        println!("Invalid frankolang");
    }

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

fn generateTestTransaction(message: &mut [u8; 180]) {

    // I know this code is garbage is will be removed
    message[0] = 0x03;
    message[73] = 0x04;
    message[73+9] = 0x02;

    let mut messageToSign: [u8; 83] = [0; 83];
    for byte in 0..83 {
        messageToSign[byte] = message[byte];
    }

    let mut csprng = rand::rngs::OsRng;
    let keypair: ed25519_dalek::Keypair = ed25519_dalek::Keypair::generate(&mut csprng);
    let signatureObj: ed25519_dalek::Signature = keypair.sign(&messageToSign);
    let signature = signatureObj.to_bytes();
    let publicKey = keypair.public.to_bytes();

    // Add signature and public key to message
    message.rotate_right(97);
    message[0] = 0x01;

    for byte in 1..65 {
        message[byte] = signature[byte-1];
    }
    for byte in 65..97 {
        message[byte] = publicKey[byte-65];
    }
}
