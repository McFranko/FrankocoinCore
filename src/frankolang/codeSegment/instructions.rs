use crate::dirs;
use crate::md5;
use crate::serde::{Serialize, Deserialize};
use crate::bincode;
use std::fs;

/// When dryrun is set to true, no changes will be made to the database. Instead it will just
/// return an Err() if the sender has insufficient funds
#[derive(Copy, Clone)]
pub struct Payment {
    pub senderBalanceEntry: BalanceEntry,
    pub recieverBalanceEntry: BalanceEntry,
    pub amount: u64,
    pub dryrun: bool
}

impl Payment {
    pub fn new(sender: [u8; 32], reciever: [u8; 32], amount: u64, dryrun: bool)
        -> Result<Payment, Box<dyn std::error::Error>>
    {
        Ok(
            Payment {
                senderBalanceEntry: BalanceEntry::new(&sender)?,
                recieverBalanceEntry: BalanceEntry::new(&reciever)?,
                amount,
                dryrun
            }
        )
    }

    pub fn send(&mut self)
        -> Result<(), Box<dyn std::error::Error>>
    {
	    if self.senderBalanceEntry.balance < self.amount {
            return Err(Box::new(InsufficientFundsError));
        }

        self.senderBalanceEntry.balance -= self.amount;
        self.recieverBalanceEntry.balance += self.amount;

        if !self.dryrun {
            self.senderBalanceEntry.save()?;
            self.recieverBalanceEntry.save()?;
        }

        Ok(())
    }
}


#[derive(Debug)]
struct InsufficientFundsError;
impl std::error::Error for InsufficientFundsError {}

impl std::fmt::Display for InsufficientFundsError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter)
        -> std::fmt::Result
    {
        write!(formatter, "Insufficient funds")
    }
}


#[derive(Serialize, Deserialize, PartialEq, Copy, Clone)]
pub struct BalanceEntry {
    publicKey: [u8; 32],
    balance: u64
}

impl BalanceEntry {
    pub fn new(publicKey: &[u8; 32])
        -> Result<BalanceEntry, Box<dyn std::error::Error>>
    {
       let fallbackBalanceEntry = BalanceEntry {
            publicKey: *publicKey,
            balance: 0
        };

        let balances = match readBalances(publicKey) {
            Ok(balances) => balances,
            Err(_) => {
                return Ok(fallbackBalanceEntry);
            }
        };

        let balanceEntryPosition = match balances
            .iter()
            .position(|balanceEntry| &balanceEntry.publicKey == publicKey)
        {
            Some(balanceEntryPosition) => balanceEntryPosition,
            None => 
            {
                return Ok(fallbackBalanceEntry);
            }
        };

        Ok(balances[balanceEntryPosition])
    }

    pub fn save(&self) 
        -> Result<(), Box<dyn std::error::Error>>
    {
        let balances = match readBalances(&self.publicKey) {
            Ok(balances) => balances,
            Err(_) => {
                let mut balances: Vec<BalanceEntry> = Vec::new();
                balances.push(self.clone());
                balances
            }
        };

        let filename = findBalanceEntryFilename(&self.publicKey);

        let balancesBuffer = bincode::serialize(&balances)?;
        fs::write(filename, balancesBuffer)?;

        Ok(())
    }
}

fn readBalances(publicKey: &[u8; 32])
    -> Result<Vec<BalanceEntry>, Box<dyn std::error::Error>>
{
        let filename = findBalanceEntryFilename(publicKey);

        let balancesBuffer: Vec<u8> = fs::read(&filename)?;

        let balances: Vec<BalanceEntry> = match bincode::deserialize(&balancesBuffer) {
            Ok(balances) => balances,
            Err(_) => {
                // Panics because if a file is unable to be deserialized, that means it's been
                // corrupted and then future blocks who rely on balances in this file cannot be
                // properly verified. If this panic occurs, the user should reset their
                // frankocoin node
                panic!(
                    format!(
                        "File {} is corrupted",
                        filename
                    )
                )
            }
        };
        Ok(balances)
}

fn findBalanceEntryFilename(publicKey: &[u8; 32])
    -> String
{
    format!(
        "{}/frankocoin/balanceEntries/{:x?}.dat",
        dirs::data_dir().unwrap().to_str().unwrap(),
        &md5::compute(publicKey)[0..2]
    )
}
