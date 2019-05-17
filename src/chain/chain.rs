use super::super::transaction;
use super::block;
use super::error;
use super::writer;

pub struct Chain {
    pub blocks: Vec<block::Block>,
}

impl Chain {

    pub fn new() -> Result<Self, error::MiningError> {
        let blocks = block::Block::genesis()?;
        Ok(Self{ blocks: vec![blocks]})
    }

    pub fn push(&mut self, data: transaction::Transaction) -> Result<(), error::MiningError> {
        let block: block::Block;
        {
            match self.blocks.last() {
                Some(prev) => {
                    block = block::Block::new(data, prev.hash())?;
                }
                None => {
                    return Err(error::MiningError::NoParent)
                }
            }
        }
        self.blocks.push(block);
        writer::save_file_async(&self);
        Ok(())
    }

     // A method that iterates over the blockchain's blocks and prints out information for each.
    pub fn traverse(&self) {
        for (i, block) in self.blocks.iter().enumerate() {
            println!("block: {}", i);
            println!("data: {:?}", block.print());
            println!()
        }
    }

}
