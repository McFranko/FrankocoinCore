use std::error::Error;

use crate::blocks::{Block, Transaction};

#[test]
fn blocks() -> Result<(), Box<dyn Error>> {
    // This doesn't have proper information or addresses and such, this should be implemented into
    // the test when it's possible to do that.
    let mut block = Block::new(
        0,
        vec![0, 0],
        [5u8; 32],
        [0u8; 16],
        transactions(),
    )?;
    println!(
        "Block {}'s hash validity: {}",
        0,
        block.check_hash(1).is_ok()
    );

    block.save()?;

    let _block = Block::from_height(0)?;

    Ok(())
}

fn transactions() -> Vec<Transaction> {
    let transaction = Transaction {
        sender: [0u8; 32],
        signature: vec![0u8; 64],
        inputs: vec![[0u8; 16]],
        outputs: vec![[0u8; 16]],
    };
    vec![transaction.clone(), transaction.clone()]
}
