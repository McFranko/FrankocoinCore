use crate::{
    bincode, dirs,
    serde::{Deserialize, Serialize},
    sha2::{Digest, Sha256},
};
use std::{error::Error, fs};

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    height: u64,
    nonce: Vec<u8>,
    hash: [u8; 32],
    merkle_root: [u8; 16],
    previous_hash: [u8; 32],
    miner_address: [u8; 32],
    data: Vec<u8>,
}

impl Block {
    #[allow(unused_variables, unreachable_code)]
    pub fn new(
        height: u64,
        nonce: Vec<u8>,
        miner_address: [u8; 32],
        data: Vec<u8>,
    ) -> Result<Block, Box<dyn Error>> {
        todo!();

        let block = Block {
            height,
            nonce,
            hash: [0u8; 32],
            merkle_root: [0u8; 16],
            previous_hash: [0u8; 32],
            miner_address,
            data,
        };

        Ok(block)
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
        let to_hash: Vec<u8> =
            vec![self.merkle_root.to_vec(), self.previous_hash.to_vec()]
                .concat();
        let mut hasher = Sha256::new();
        hasher.update(to_hash);
        let hash = &hasher.finalize()[..];

        for byte_number in 0..leading_zeros {
            if hash[byte_number] != 0u8 {
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
