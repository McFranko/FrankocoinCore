use crate::{
    bincode,
    ed25519_dalek::{PublicKey, Signature, Verifier},
    serde::{Deserialize, Serialize},
};

use std::{convert::TryFrom, error::Error};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub sender: [u8; 32],
    pub signature: Vec<u8>,
    pub inputs: Vec<[u8; 16]>,
    pub outputs: Vec<[u8; 16]>,
}

impl Transaction {
    pub fn check_signature(&self) -> Result<(), Box<dyn Error>> {
        let signature = Signature::try_from(&self.signature[..])?;
        let public_key = PublicKey::from_bytes(&self.sender)?;
        let message = bincode::serialize(&[&self.inputs, &self.outputs])?;

        public_key.verify(&message, &signature)?;
        Ok(())
    }
}

impl From<Transaction> for Vec<u8> {
    fn from(transaction: Transaction) -> Self {
        // unwrap() is okay here, as serialization can only fail in very specific situations. I
        // will give 20$ worth of frankocoin to anyone who happens to come across those situations.
        bincode::serialize(&transaction).unwrap()
    }
}
