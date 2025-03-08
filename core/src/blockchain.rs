use crate::block;

pub struct BlockChain {
    pub blocks: Vec<block::Block>,
}

impl BlockChain {
    ///困难度
    const DIFFICULTY: usize = 4;
    pub fn add_block(&mut self, data: String) {
        let pre_block = &self.blocks[self.blocks.len() - 1];
        let new_block = block::Block::new_block(data, pre_block.hash.clone(), Self::DIFFICULTY);
        self.blocks.push(new_block);
    }

    fn new_genesis_block() -> block::Block {
        block::Block::new_block("The genesis block".to_string(), "".to_string(),Self::DIFFICULTY)
    }

    pub fn new_block_chain() -> BlockChain {
        BlockChain {
            blocks: vec![BlockChain::new_genesis_block()],
        }
    }
}
