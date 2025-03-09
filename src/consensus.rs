use crate::blockchain::Block;

pub struct Consensus {
    _difficulty: u32,
}

impl Consensus {
    pub fn new() -> Self {
        Self { _difficulty: 0 }
    }

    pub fn get_hash(&self, block: &Block) -> String {
        let _hash_input = format!("{}{}{}", block.timestamp, block.prev_hash, block.nonce);
        // TODO: Covert Block into hash
        String::new()
    }

    #[allow(dead_code)]
    pub fn verify(&self) {
        // TODO: Verify the Blockchain
    }

    pub fn calculate(&self /* TODO: BlockChain, New Block */) -> u32 {
        // TODO: Check the difficulty and adjust
        // TODO: Get the last Block hash
        // TODO: Calculate the hash in new Block
        // TODO: Return the nonce
        0
    }
}
