extern crate bincode;
extern crate serde;
extern crate ed25519_dalek;
extern crate merkle_tree;
extern crate sha2;

use std::{
    fs,
    path::Path
};

pub mod blocks;

pub fn init_blockchain_directory(
    blockchain_directory: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(blockchain_directory).exists() {
        fs::create_dir(blockchain_directory)?;
    }

    Ok(())
}

