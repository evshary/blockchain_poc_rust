use bincode::serialize;
use sha2::{Digest, Sha256};

use crate::blockchain::Block;

// Enable TEST_MODE to make generating the block stably
const TEST_MODE: bool = true;

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

        format!("{result:x}")
    }

    #[allow(dead_code)]
    pub fn verify(&self) {
        // TODO: Verify the Blockchain
    }

    pub fn adjust(&mut self, blocks: &[Block]) {
        if TEST_MODE {
            // Fix the difficulty to 3 but sleep for 5 seconds to avoid too fast
            self.difficulty = 3;
            std::thread::sleep(std::time::Duration::from_secs(5));
        }

        // Check the timestamp: Only for debug
        for block in blocks.iter() {
            tracing::trace!("Timestamp: {}", block.timestamp);
        }

        // Check the difficulty and adjust
        let total_diff: u128 = blocks
            .windows(2) // Create overlapping pairs
            .map(|pair| pair[1].timestamp - pair[0].timestamp) // Compute differences
            .sum();

        let avg_diff = total_diff as f64 / (blocks.len() as f64 - 1.0);
        tracing::debug!("Average time: {}", avg_diff);
        if avg_diff.is_nan() {
            return;
        }
        tracing::debug!("Original Difficulty: {}", self.difficulty);
        if TIMESTAMP_DIFFERENCE as f64 > avg_diff {
            // Increase the difficulty
            self.difficulty += 1;
        } else {
            // Decrease the difficulty
            self.difficulty -= 1;
        }
        tracing::debug!("New Difficulty: {}", self.difficulty);
    }

    pub fn calculate(&self, block: &mut Block) {
        // Calculate the hash
        loop {
            let hash = self.get_hash(block);
            tracing::trace!("Hash: {}", hash);
            if hash.starts_with(&"0".repeat(self.difficulty as usize)) {
                block.hash = hash;
                tracing::debug!("Final Hash: {}", block.hash);
                break;
            }
            block.nonce += 1;
        }
    }
}
