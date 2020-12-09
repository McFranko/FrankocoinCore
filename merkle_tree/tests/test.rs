extern crate sha2;

use sha2::{Digest, Sha224};
use std::error::Error;

use merkle_tree::MerkleTree;

#[test]
fn make_tree() -> Result<(), Box<dyn Error>> {
    let test_data: Vec<Vec<u8>> = vec![vec![0; 2]; 13];

    let merkle_tree = MerkleTree::new(test_data);
    println!("{:#x?}", merkle_tree);

    let hash = &Sha224::digest(&[0; 2])[..];
    merkle_tree.get_proof(hash);
    Ok(())
}

#[test]
fn get_proof() -> Result<(), Box<dyn Error>> {
    let test_data: Vec<Vec<u8>> = vec![vec![0; 2]; 5];

    let merkle_tree = MerkleTree::new(test_data);

    let hash = &Sha224::digest(&[0; 2])[..];
    merkle_tree.get_proof(hash);
    
    Ok(())
}
