// Frankolang interpeter
// This will always be on for the duration of the program on another thread
// Communication will be done via a socket.
#![allow(dead_code)]
#![allow(non_snake_case)]
extern crate ed25519_dalek;

use crate::server;
use std::io::Write;
use std::io::Read;

pub struct Frankolang;

impl Frankolang {
    // Start the interpreter
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
    fn interpretFrankolang(code: &[u8], dryrun: bool) -> bool{
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


                    // Make a buffer from the code (excluding startsig instruction)
                    let mut message: Vec<u8> = Vec::new();
                    let mut isThereAnEnd = false;
                    for n in currentByte+97..code.len() {
                        message.push(code[n]);
                        if code[n] == 0x02 {
                            isThereAnEnd = true;
                            break;
                        }
                    }

                    if !isThereAnEnd {
                        println!("Frankolang Interpreter: Syntax error on byte {}, Instruction as decimal {}\nCould not find 0x02 instruction.", currentByte, code[currentByte]);
                        return false;
                    }

                    // Find the signature and read it to a buffer
                    let mut signature: [u8; 64] = [0; 64];
                    for signatureBufferByte in currentByte+1..currentByte+65 {
                        signature[signatureBufferByte -(currentByte+1)] = code[signatureBufferByte];
                    }

                    // Find public key and read it to a buffer
                    let mut publicKey: [u8; 32] = [0; 32];
                    for pkBufferByte in currentByte+65..currentByte+97 {
                        publicKey[pkBufferByte - (currentByte+65)] = code[pkBufferByte];
                    }

                    // Verify the signature
                    let publicKeyObject = ed25519_dalek::PublicKey::from_bytes(&publicKey).unwrap();
                    let signatureObject = ed25519_dalek::Signature::from_bytes(&signature).unwrap();
                    let verified = publicKeyObject.verify(&message, &signatureObject);
                    print!("Signature is {:?}\n", verified.is_ok());

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
                    // match dryrun {   This can be used for once the interpreter does
                    //     true => {}   more than just say what instruction it was given
                    //     false => {
                                // interpretation will go here
                    //     }
                    // }
                    currentByte+=73;
                }
                0x04 => {
                    // fee is 9 bytes
                    println!("Frankolang Interpreter: Interpreting fee instruction");
                    currentByte+=9;
                }
                _ => {
                    println!("Frankolang Interpreter: Syntax error on byte {}, Instruction as decimal {}", currentByte, code[currentByte]);
                    return false
                }
            }
        }

    }
}


fn handle(mut socket: std::net::TcpStream) {
    loop {
        let mut req: [u8; 200] = [0; 200]; // It would be better if this was a vector so it didn't use up so much ram, but i couldn't figure it out and I don't wanna spend too much time on it
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
            0x11 => { // Real run
                req.rotate_left(1);
                Frankolang::interpretFrankolang(&req, false);
            }
            0x12 => { // Dry run
                req.rotate_left(1);
                if Frankolang::interpretFrankolang(&req, true) {
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
            _ => {
                println!("Frankolang Interpreter: {} is not a command", req[0]);
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(100)); // Only checks every 100 milliseconds to not use up unnecissary computing power
    }
}
