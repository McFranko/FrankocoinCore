use std::{error::Error, fs};

use blockchain::{
    blocks::{Block, Transaction},
    init_blockchain_directory,
};

#[test]
fn blocks() -> Result<(), Box<dyn Error>> {
    // The directory used to store blocks for testing purposes. Normally the director would be the
    // static variable blockchain_directory, but this test would overwrite anything that was
    // currently in that directory if blockchain_directory was used for testing.
    let blockchain_directory = "./blockchain_directory";
    init_blockchain_directory(&blockchain_directory)?;

    // This doesn't have proper information or addresses and such, this should be implemented into
    // the test when it's possible to do that.
    let block = Block::new(
        0,
        vec![0, 0],
        [5u8; 32],
        [0u8; 16],
        construct_transactions(),
    )?;
    println!(
        "Block {}'s hash validity: {}",
        0,
        block.check_hash(0).is_ok(),
    );
    println!(
        "Block {}'s transaction validity: {}",
        0,
        block.check_transactions().is_ok(),
    );

    block.save(&blockchain_directory)?;

    let _block = Block::from_height(0, &blockchain_directory)?;

    // Removes the directory after the test is finished in order to declutter the workspace
    fs::remove_dir_all(&blockchain_directory)?;

    Ok(())
}

fn construct_transactions() -> Vec<Transaction> {
    let transaction = Transaction {
        sender: [0u8; 32],
        signature: vec![0u8; 64],
        inputs: vec![[0u8; 16]],
        outputs: vec![[0u8; 16]],
    };
    vec![transaction.clone(), transaction.clone()]
}
