#![allow(non_snake_case, dead_code)]
#![cfg(test)]

mod blocks;
mod server;

#[test]
fn init_frankocoin_directory_test() {
    crate::init_frankocoin_directory().unwrap();
}
