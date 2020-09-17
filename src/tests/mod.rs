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
