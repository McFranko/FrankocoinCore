#![allow(non_snake_case)]
mod server;
use std::io::Write;
use std::io::Read;


fn main() {
    // Create server and make parameters
    let server = server::TcpServer {
        ipAddress: String::from("localhost:8888"),
        handler: connectionHandler
    };
    // Start server on a new thread
    std::thread::spawn(move || {
        server.start();
    });

    // I had it just sleep for a minute now so I can test the server without the program just closing on me
    std::thread::sleep(std::time::Duration::from_secs(60));
}



fn connectionHandler(mut stream: std::net::TcpStream) {
    // Read request
    let mut req: [u8; 512] = [0; 512];
    let err = stream.read(&mut req);
    // Handle errors
    if err.is_err() {
        eprintln!("Could not read stream");
        return;
    }

    // Find what type of request it is
    let reqStr = std::str::from_utf8(&req);

    // I know I could probably make it a little faster by not converting it a string
    // and just comparing the sent buffer against buffers of the keyworks below, but
    // this is just a lot easier. Maybe someone else can do that if they really feel
    // like it.

    // Handle errors
    if reqStr.is_err() {
        eprintln!("Could not convert request to string");
        return;
    }
    let reqSplit: Vec<&str> = reqStr.unwrap().splitn(1, '\n').collect();

    match reqSplit[0] {
        "newBlock" => {
            // Check block signatures and PoW, if adds up execute frankolang
        },
        "newCodeSection" => {
            // Check the code section sent, and if the signature and syntax is correct, add it to the unexecuted code list
        },
        "reqBlock" => {
            // Find what block they are requesting and add write it to the stream
        },
        "reqUnexec" => {
            // Send all unexecuted code (frankolang code)
        },
        "reqExec" => {
            // Send all executed code (frankolang code)
        },
        // Unexecuted Code is code that has it's signature verified, but is waiting to be added to a block by miners
        // Executed code is code that has been interpreted and is part of a valid block (pretty self explanitory)
        &_ => return
    }
}