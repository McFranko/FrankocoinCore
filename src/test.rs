#![allow(non_snake_case)]
use ed25519_dalek;
use rand;

#[path = "frankolang/mod.rs"] mod frankolang;

pub fn message() -> bool {
    let mut message: [u8; 180] = [0; 180];

    message[0] = 0x03;
    message[73] = 0x04;
    message[73+9] = 0x02;
    
    // signature testing 
    // Generating the signature is just used to check if the interpreter is working for now
    
    let mut messageToSign: [u8; 83] = [0; 83];
    
    for byte in 0..83 {
        messageToSign[byte] = message[byte];
    }
    
    // This is just generating a signature on the message in order to test the signature checking on the Interpreter
    
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
    
    frankolang::interpretFrankolang(&message, true)
}