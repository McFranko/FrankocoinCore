use crate::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub sender: [u8; 32],
    pub signature: Vec<u8>,
    pub inputs: Vec<[u8; 16]>,
    pub outputs: Vec<[u8; 16]>,
}

impl From<Transaction> for Vec<u8> {
    fn from(transaction: Transaction) -> Self {
        // unwrap() is okay here, as serialization can only fail in very specific situations. I
        // will give 20$ worth of frankocoin to anyone who happens to come across those situations.
        bincode::serialize(&transaction).unwrap()
    }
}
