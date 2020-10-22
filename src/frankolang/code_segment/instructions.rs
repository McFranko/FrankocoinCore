use crate::bincode;
use crate::dirs;
use crate::md5::{Digest, Md5};
use crate::serde::{Deserialize, Serialize};
use std::fs;

/// When dryrun is set to true, no changes will be made to the database. Instead it will just
/// return an Err() if the sender has insufficient funds
#[derive(Copy, Clone)]
pub struct Payment {
    pub sender_balance_entry: BalanceEntry,
    pub reciever_balance_entry: BalanceEntry,
    pub amount: u64,
    pub dryrun: bool,
}

impl Payment {
    pub fn new(
        sender: [u8; 32],
        reciever: [u8; 32],
        amount: u64,
        dryrun: bool,
    ) -> Result<Payment, Box<dyn std::error::Error>> {
        Ok(Payment {
            sender_balance_entry: BalanceEntry::new(&sender)?,
            reciever_balance_entry: BalanceEntry::new(&reciever)?,
            amount,
            dryrun,
        })
    }

    pub fn send(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.sender_balance_entry.balance < self.amount {
            return Err(InsufficientFundsError::new());
        }

        self.sender_balance_entry.balance -= self.amount;
        self.reciever_balance_entry.balance += self.amount;

        if !self.dryrun {
            self.sender_balance_entry.save()?;
            self.reciever_balance_entry.save()?;
        }

        Ok(())
    }
}

#[derive(Debug)]
struct InsufficientFundsError;
impl std::error::Error for InsufficientFundsError {}

impl std::fmt::Display for InsufficientFundsError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "Insufficient funds")
    }
}

impl InsufficientFundsError {
    fn new() -> Box<InsufficientFundsError> {
        Box::new(InsufficientFundsError)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Copy, Clone)]
pub struct BalanceEntry {
    public_key: [u8; 32],
    balance: u64,
}

impl BalanceEntry {
    pub fn new(
        public_key: &[u8; 32],
    ) -> Result<BalanceEntry, Box<dyn std::error::Error>> {
        let fallback_balance_entry = BalanceEntry {
            public_key: *public_key,
            balance: 0,
        };

        let balances = match read_balances(public_key) {
            Ok(balances) => balances,
            Err(_) => {
                return Ok(fallback_balance_entry);
            }
        };

        let balance_entry_position = match balances
            .iter()
            .position(|balance_entry| &balance_entry.public_key == public_key)
        {
            Some(balance_entry_position) => balance_entry_position,
            None => {
                return Ok(fallback_balance_entry);
            }
        };

        Ok(balances[balance_entry_position])
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let balances = match read_balances(&self.public_key) {
            Ok(balances) => balances,
            Err(_) => {
                let mut balances: Vec<BalanceEntry> = Vec::new();
                balances.push(self.clone());
                balances
            }
        };

        let filename = find_balance_entry_filename(&self.public_key);

        let balances_buffer = bincode::serialize(&balances)?;
        fs::write(filename, balances_buffer)?;

        Ok(())
    }
}

fn read_balances(
    public_key: &[u8; 32],
) -> Result<Vec<BalanceEntry>, Box<dyn std::error::Error>> {
    let filename = find_balance_entry_filename(public_key);

    let balances_buffer: Vec<u8> = fs::read(&filename)?;

    let balances: Vec<BalanceEntry> =
        match bincode::deserialize(&balances_buffer) {
            Ok(balances) => balances,
            Err(_) => {
                // Panics because if a file is unable to be deserialized, that means it's been
                // corrupted and then future blocks who rely on balances in this file cannot be
                // properly verified. If this panic occurs, the user should reset their
                // frankocoin node
                panic!(format!("File {} is corrupted", filename))
            }
        };
    Ok(balances)
}

fn find_balance_entry_filename(public_key: &[u8; 32]) -> String {
    let mut hasher = Md5::new();
    hasher.update(public_key);
    let hash = &hasher.finalize()[0..2];
    format!(
        "{}/frankocoin/balanceEntries/{:x?}.dat",
        dirs::data_dir().unwrap().to_str().unwrap(),
        hash
    )
}
