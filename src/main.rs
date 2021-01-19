#![allow(dead_code)]
extern crate bincode;
extern crate dirs;
extern crate ed25519_dalek;
extern crate md5;
extern crate serde;
extern crate sha2;
extern crate typetag;
#[macro_use]
extern crate lazy_static;

extern crate merkle_tree;

mod blocks;
mod server;
mod threadpool;

#[cfg(test)]
mod tests;

use std::fs;
use std::path::Path;

lazy_static! {
    static ref FRANKOCOIN_DIRECTORY: String = {
        format!("{}/frankocoin", dirs::data_dir().unwrap().to_str().unwrap())
    };
}

fn main() {}

/// Creates the directories for frankocoin (if they don't already exist)
fn init_frankocoin_directory(
    frankocoin_directory: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(frankocoin_directory).exists() {
        fs::create_dir(frankocoin_directory)?;
    }

    let blocks_directory = format!("{}/blocks", frankocoin_directory);
    if !Path::new(&blocks_directory).exists() {
        fs::create_dir(&blocks_directory)?;
    }

    Ok(())
}

fn clone_into_array<A, T>(slice: &[T]) -> A
where
    A: Default + AsMut<[T]>,
    T: Copy,
{
    let mut a = A::default();
    A::as_mut(&mut a).copy_from_slice(slice);
    a
}
