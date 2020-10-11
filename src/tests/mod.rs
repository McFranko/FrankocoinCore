#![allow(non_snake_case, dead_code)]
#![cfg(test)]

// In order for tests to work you must do `cargo run` first to initialize the frankocoin directory
// (~/.frankocoin on UNIX systems)

#[test]
fn initializeFrankocoinDirectoryTest() {
    crate::initializeFrankocoinDirectory().unwrap();
}

mod frankolangInterpreter;
mod server;

pub fn testFrankolang() -> [u8; 149] {
    let mut codeToSign = [0u8; 51];
    codeToSign[0] = 0x03;
    // reciever public key and amount is left blank for now, as those features
    // aren't implemented yet

    codeToSign[41] = 0x04;
    // fee is left empty because that hasn't been implemented yet

    codeToSign[50] = 0x02;

    // Adding keys and signature
    let mut code = [0u8; 149];
    code[0] = 0x01;
    let mut csprng = rand::rngs::OsRng {};
    let keypair = ed25519_dalek::Keypair::generate(&mut csprng);
    for (index, byte) in code[1..33].iter_mut().enumerate() {
        *byte = keypair.public.to_bytes()[index];
    }

    let signature = keypair.sign(&codeToSign);
    for (index, byte) in code[33..97].iter_mut().enumerate() {
        *byte = signature.to_bytes()[index];
    }

    for (index, byte) in code[97..148].iter_mut().enumerate() {
        *byte = codeToSign[index];
    }

    code[148] = 0x0f;

    code
}
