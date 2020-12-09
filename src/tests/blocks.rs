use crate::blocks::Block;
use crate::tests::test_frankolang;

#[test]
fn blocks() -> Result<(), Box<dyn std::error::Error>> {
    for height in 0..5 {
        let mut block = Block::new(
            height,
            Vec::new(),
            [1u8; 32],
            test_frankolang().to_vec(),
        )?;
        println!("Block {}'s hash validity: {}", height, block.check_hash(1).is_ok());

        block.save().unwrap();
    }
    Ok(())
}
