#![allow(non_snake_case)]

use crate::md5;
use crate::serde::{Serialize, Deserialize};
use crate::bincode;
use std::fs;
use std::io::{Read, Write};
use std::path::Path

pub struct Payment
{
    sender: [u8; 32],
    reciever: [u8; 32],
    amount: u64,
    fee: u64
}

impl Payment
{
    pub fn send(&self) -> Result<(), Box<dyn std::error::Error>>
    {
        let senderBalanceEntry = match BalanceEntry::fromKey(&self.sender)
        {
            Ok(senderBalanceEntry) => senderBalanceEntry,
            Err(_) => {
                return Err(Payment::insufficientFundsError());
            }
        };

	if senderBalanceEntry.balance < self.amount + self.fee
        {
            return Err(Payment::insufficientFundsError());
        }

        Ok(())
    }

    // This is a function instead of a variable so that the heap allocation
    // only happens if there is an error
    fn insufficientFundsError() -> Box<std::io::Error>
    {
        Box::new(
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "Insufficient funds"
            )
        )
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub struct BalanceEntry
{
    publicKey: [u8; 32],
    balance: u64
}

impl BalanceEntry
{
    pub fn fromKey(publicKey: &[u8; 32])
        -> Result<BalanceEntry, Box<dyn std::error::Error>>
    {
        // TODO: Implement a caching system so that active public key's balance
        // entries can be stored in memory.
        let fallbackBalanceEntry = BalanceEntry
        {
            publicKey: *publicKey,
            balance: 0
        };

        let token = format!("{:x?}", &md5::compute(publicKey)[0..3]);

        let file: Vec<u8> = match fs::read(token) {
            Ok(file) => file,
            Err(_) => {
                return Ok(fallbackBalanceEntry);
            }
        }

        let balances: Vec<BalanceEntry> = bincode::deserialize(&file)?;

        let balanceEntryPosition = match balances.iter().position(
            |balanceEntry| &balanceEntry.publicKey == publicKey
        ) {
            Some(balanceEntryPosition) => balanceEntryPosition,
            None => {
                return Ok(fallbackBalanceEntry);
            }
        };

        Ok(balances[balanceEntryPosition])
    }
}
