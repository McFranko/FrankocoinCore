mod transaction;

use crate::{
    bincode, dirs,
    merkle_tree::MerkleTree,
    serde::{Deserialize, Serialize},
    sha2::{Digest, Sha256},
};

use std::{error::Error, fs};

pub use transaction::Transaction;

/// A block of transactions, and it's corresponding header data.
///
/// The block hash is calculated buy concatenating the previous hash, the merkle root, the
/// height, and then the nonce.
#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    height: u64,
    nonce: Vec<u8>,
    hash: [u8; 32],
    previous_hash: [u8; 32],
    miner_address: [u8; 16],
    merkle_tree: MerkleTree,
    transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(
        height: u64,
        nonce: Vec<u8>,
        previous_hash: [u8; 32],
        miner_address: [u8; 16],
        transactions: Vec<Transaction>,
    ) -> Result<Block, Box<dyn Error>> {
        let merkle_tree = MerkleTree::new(&transactions);

        let to_hash =
            [previous_hash.into(), merkle_tree.root.into(), height.to_le_bytes().into(), nonce.clone()]
                .concat();
        let hash = Sha256::digest(&to_hash).into();

        Ok(Block {
            height,
            nonce,
            hash,
            previous_hash,
            miner_address,
            merkle_tree,
            transactions,
        })
    }

    pub fn from_height(height: u64) -> Result<Block, Box<dyn Error>> {
        let file_name = format!(
            "{}/frankocoin/blocks/block{}.dat",
            dirs::data_dir().unwrap().to_str().unwrap(),
            height
        );
        let file = fs::read(file_name)?;
        let block: Block = bincode::deserialize(&file)?;
        Ok(block)
    }

    pub fn check_hash(
        &mut self,
        leading_zeros: usize,
    ) -> Result<(), Box<dyn Error>> {
        let to_hash: Vec<u8> = vec![
            self.previous_hash.to_vec(),
            self.merkle_tree.root.to_vec(),
            self.height.to_le_bytes().into(),
            self.nonce.clone()
        ]
        .concat();
        let hash = &Sha256::digest(&to_hash)[..];

        for byte_number in 0..leading_zeros {
            if hash[byte_number] != 0x00 {
                return Err(InvalidHashError::new(hash));
            }
        }

        if hash != self.hash {
            return Err(InvalidHashError::new(hash));
        }

        Ok(())
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let file_name = format!(
            "{}/frankocoin/blocks/block{}.dat",
            dirs::data_dir().unwrap().to_str().unwrap(),
            self.height
        );
        fs::write(file_name, bincode::serialize(self)?)?;
        Ok(())
    }
}

#[derive(Debug)]
struct InvalidHashError {
    hash: Vec<u8>,
}
impl Error for InvalidHashError {}
impl std::fmt::Display for InvalidHashError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "Invalid block hash {:x?}", &self.hash[..])
    }
}

impl InvalidHashError {
    fn new<T: Into<Vec<u8>>>(hash: T) -> Box<InvalidHashError> {
        Box::new(InvalidHashError { hash: hash.into() })
    }
}
