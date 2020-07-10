#![allow(non_snake_case)]
extern crate ed25519_dalek;
extern crate rand;
mod header;
mod frankolang;
mod server;

use header::*;
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


    // Testing interpreter /*

    // make message
    let mut message: [u8; 180] = [0; 180];
    message[0] = 0x03; // payto
    message[73] = 0x04; // fee
    message[73+9] = 0x02; // endsig



    // Testing signature
    // Generating the signature is just used to check if the interpreter is working for now
    // Yes I know this code is absolutely terrible, it will be removed later

    let mut messageToSign: [u8; 83] = [0; 83];
    for byte in 0..83 {
        messageToSign[byte] = message[byte];
    }

    // This is just generating a signature on the message in order to test the signature checking on the Interpreter
    // This will be removed later
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

    let codeIsGood = frankolang::interpretFrankolang(&message, true);

    if codeIsGood {
        println!("Valid frankolang");
        frankolang::interpretFrankolang(&message, false);
    } else {
        println!("Invalid frankolang");
    }

    //      *\

    // I had it just sleep for a minute now so I can test the server without the program just closing on me
    std::thread::sleep(std::time::Duration::from_secs(600));
}


fn connectionHandler(mut socket: std::net::TcpStream) {
    // Read request
    let mut request: [u8; 1048576] = [0; 1048576];
    socket.read(&mut request).unwrap();

    match splitBufferAt(&request, 0x0a, 1)[0] { // 0x0a is ASCII newline or \n
        b"newBlock" => {
            // socket.write(newBlock(splitRequest[1]));
        },
        &_ => return
    }

}
