use super::block;
use super::error;

pub struct Chain {
    blocks: Vec<block::Block>,
}

impl Chain {

    pub fn new() -> Result<Self, error::MiningError> {
        let blocks = block::Block::genesis()?;
        Ok(Self{ blocks: vec![blocks]})
    }

    pub fn push(&mut self, data: &str) -> Result<(), error::MiningError> {
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
