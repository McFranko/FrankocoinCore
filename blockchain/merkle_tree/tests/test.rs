extern crate sha2;

use std::{convert::TryInto, error::Error};

use sha2::{Digest, Sha224};

use merkle_tree::MerkleTree;

#[test]
fn test() -> Result<(), Box<dyn Error>> {
    let test_data: Vec<Vec<u8>> = vec![
        vec![0x0; 2],
        vec![0x0a; 5],
        vec![0xa2; 2],
        vec![0x1; 12],
        vec![0xfe; 27],
    ];

    let merkle_tree = MerkleTree::new(test_data);

    let hash = &Sha224::digest(&[0x0a; 5])[..];
    let merkle_proof = merkle_tree
        .get_proof(hash.try_into().unwrap())
        .ok_or("Couldn't get merkle proof")?;
    assert!(merkle_proof.is_proof(&merkle_tree.root));
    assert!(!merkle_proof.is_proof(&[0u8; 28]));

    Ok(())
}
