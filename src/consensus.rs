use bincode::serialize;
use sha2::{Digest, Sha256};

use crate::blockchain::Block;

// 10 minutes = 10 * 60 * 1000 milliseconds
const TIMESTAMP_DIFFERENCE: u128 = 600000;

pub struct Consensus {
    difficulty: u32,
}

impl Consensus {
    pub fn new() -> Self {
        Self { difficulty: 0 }
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

    pub fn adjust(&mut self, blocks: &mut Vec<Block>) {
        // Check the difficulty and adjust
        for block in blocks.iter() {
            println!("timestamp: {}", block.timestamp);
        }
        let total_diff: u128 = blocks
            .windows(2) // Create overlapping pairs
            .map(|pair| pair[1].timestamp - pair[0].timestamp) // Compute differences
            .sum();

        let avg_diff = total_diff as f64 / (blocks.len() as f64 - 1.0);
        println!("Average time: {}", avg_diff);
        if avg_diff.is_nan() {
            return;
        }
        println!("Original Difficulty: {}", self.difficulty);
        if TIMESTAMP_DIFFERENCE as f64 > avg_diff {
            // Increase the difficulty
            self.difficulty += 1;
        } else {
            // Decrease the difficulty
            self.difficulty -= 1;
        }
        println!("New Difficulty: {}", self.difficulty);
    }

    pub fn calculate(&self, block: &mut Block) {
        // Calculate the hash
        loop {
            let hash = self.get_hash(block);
            //println!("Hash: {}", hash);
            if hash.starts_with(&"0".repeat(self.difficulty as usize)) {
                block.hash = hash;
                println!("Final Hash: {}", block.hash);
                break;
            }
            block.nonce += 1;
        }
    }
}
