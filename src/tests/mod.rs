#![allow(non_snake_case, dead_code)]
#![cfg(test)]

// In order for tests to work you must do `cargo run` first to initialize the frankocoin directory
// (~/.frankocoin on UNIX systems)

#[test]
fn init_frankocoin_directory_test() {
    crate::init_frankocoin_directory().unwrap();
}

mod frankolang_interpreter;
mod server;

pub fn test_frankolang() -> [u8; 149] {
    let mut code_to_sign = [0u8; 51];
    code_to_sign[0] = 0x03;
    // reciever public key and amount is left blank for now, as those features
    // aren't implemented yet

    code_to_sign[41] = 0x04;
    // fee is left empty because that hasn't been implemented yet

    code_to_sign[50] = 0x02;

    // Adding keys and signature
    let mut code = [0u8; 149];
    code[0] = 0x01;
    let mut csprng = rand::rngs::OsRng {};
    let keypair = ed25519_dalek::Keypair::generate(&mut csprng);
    for (index, byte) in code[1..33].iter_mut().enumerate() {
        *byte = keypair.public.to_bytes()[index];
    }

    let signature = keypair.sign(&code_to_sign);
    for (index, byte) in code[33..97].iter_mut().enumerate() {
        *byte = signature.to_bytes()[index];
    }

    for (index, byte) in code[97..148].iter_mut().enumerate() {
        *byte = code_to_sign[index];
    }

    code[148] = 0x0f;

    code
}
