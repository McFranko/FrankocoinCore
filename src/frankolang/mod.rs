// Frankolang interpeter
// This will always be on for the duration of the program on another thread
// Communication will be done via a socket.
#![allow(dead_code)]
use crate::server;
use std::io::Write;
use std::io::Read;

pub fn startFrankolangInterpreter() {
    // Starts the Frankolang interpreter
    let socket = server::TcpServer {
        ipAddress: String::from("localhost:8354"),
        handler: handle
    };
    std::thread::spawn(move || {
        println!("{}", socket.start(1));
    });   
}


// Interpreting is right here. Will return true if it goes smoothly and false if it didn't
fn interpretFrankolang(code: &[u8]) -> bool{
    let mut currentByte = 0; // The byte with the first part of the instruction
    
    loop {
        // Find current instruction
        match code[currentByte] {
            // execute instructions
            0x0f => {
                // ends the interpreted code
                println!("Frankolang Interpreter: Finished interpreting");
                return true;
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
                currentByte+=9;
            }
            _ => {
                println!("Frankolang Interpreter: Syntax error on instruction {}\nInstruction as decimal {}", currentByte, code[currentByte]);
                return false
            }
        }
    }
    
}

fn handle(mut socket: std::net::TcpStream) {
    loop {
        let mut req: [u8; 1048576] = [0; 1048576]; // It would be better if this was a vector so it didn't use up so much ram, but i couldn't figure it out and I don't wanna spend too much time on it
        let err = socket.read(&mut req);
        match err {
            Err(e) => {
                eprintln!("Frankolang Interpreter: Could not read from stream.\nError: {}", e);
                return;
            },
            _ =>  {}
        }
        // Execute request
        match req[0] {
            0x11 => {
                req.rotate_left(1);
                interpretFrankolang(&req);
            }
            0x12 => {
                req.rotate_left(1);
                if interpretFrankolang(&req) {
                    let result: [u8; 1] = [1];
                    let err = socket.write(&result);
                    match err {
                        Err(e) => eprintln!("Frankolang Interpreter: Could not write to socket. Error Message: {}", e),
                        _ =>  {}
                    }
                } else {
                    let result: [u8; 1] = [0];
                    let err = socket.write(&result);
                    match err {
                        Err(e) => eprintln!("Frankolang Interpreter: Could not write to socket. Error Message: {}", e),
                        _ =>  {}
                    }
                }
            }
            0xff => {
                break;
            }
            _ => {
                println!("Frankolang Interpreter: {} is not a command", req[0]);
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(250)); // Only checks every 250 milliseconds to ensure
    }
}