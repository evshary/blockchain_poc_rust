use bincode::serialize;
use sha2::{Digest, Sha256};

use crate::blockchain::Block;

pub struct Consensus {
    _difficulty: u32,
}

impl Consensus {
    pub fn new() -> Self {
        Self { _difficulty: 0 }
    }

    pub fn get_hash(&self, block: &Block) -> String {
        let encoded: Vec<u8> = serialize(&block).unwrap();

        // Sha256
        let mut hasher = Sha256::new();
        hasher.update(encoded);
        let result = hasher.finalize();

        format!("{:x}", result)
    }

    #[allow(dead_code)]
    pub fn verify(&self) {
        // TODO: Verify the Blockchain
    }

    pub fn calculate(&self, block: &mut Block) {
        // TODO: Check the difficulty and adjust

        // Calculate the hash
        loop {
            let hash = self.get_hash(block);
            println!("Hash: {}", hash);
            if hash.starts_with("0") {
                block.hash = hash;
                break;
            }
            block.nonce += 1;
        }
    }
}
