// Frankolang interpeter
// This will always be on for the duration of the program on another thread
// Communication will be done via a socket.
#![allow(dead_code)]
use crate::server;
use std::io::Read;

pub fn startFrankolangInterpreter() {
    // Starts the Frankolang interpreter
    let socket = server::TcpServer {
        ipAddress: String::from("localhost:8354"),
        handler: handle
    };
    std::thread::spawn(move || {
        println!("{}", socket.start(2));
    });   
}


// Interpreting is right here
fn interpretFrankolang(code: &[u8]) {
    // Find instructions
    let mut finished = false;
    let mut currentByte = 0; // The byte with the first part of the instruction
    while !finished {
        match code[currentByte] {
            // execute instructions
            0x0f => {
                // ends the interpreted code
                finished = true;
            }
            0x01 => {
                // startsig is 97 bytes long
                println!("Frankolang Interpreter: Interpreting startsig instruction");
                currentByte+=97;
            }
            0x02 => {
                // endsig is one byte
                println!("Frankolang Interpreter: Interpreting endsig instruction");
                currentByte+=1;
            }
            0x03 => {
                // payto is 73 bytes
                println!("Frankolang Interpreter: Interpreting payto instruction");
                currentByte+=73;
            }
            0x04 => {
                // fee is 9 bytes
                println!("Frankolang Interpreter: Interpreting fee instruction");
            }
            _ => {
                println!("Frankolang Interpreter: Syntax error");
            }
        }
    }
    
}

fn handle(mut stream: std::net::TcpStream) {
    // Read request
    let mut req: [u8; 2097152] = [0; 2097152]; // It would be better if this was a vector so it didn't use up so much ram, but i couldn't figure it out and I don't wanna spend too much time on it
    let err = stream.read(&mut req);
    if err.is_err() {
        eprintln!("Frankolang Interpreter: Could not read stream");
        return;
    }

    // Execute request
    match req[0] {
        0x11 => {
            req.rotate_left(1);
            interpretFrankolang(&req);
        }
        _ => {
            println!("Frankolang Interpreter: {} is not a command", req[0]);
        }
    }
}