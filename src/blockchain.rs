use std::{
    sync::{Mutex, RwLock},
    time,
};

use serde::{Deserialize, Serialize};

use crate::consensus::Consensus;

const TRANSACTION_PER_BLOCK: usize = 32;
const MINER_INITIAL_REWARD: u64 = 50;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub fee: u64,
    pub message: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Block {
    pub timestamp: u128,
    pub prev_hash: String,
    pub hash: String,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(prev_hash: String, transactions: Vec<Transaction>) -> Block {
        // Create timestamp
        let timestamp = time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        Block {
            timestamp,
            prev_hash,
            hash: String::from(""),
            nonce: 0,
            transactions,
        }
    }
}

pub struct Blockchain {
    pub blocks: RwLock<Vec<Block>>,
    pub mining_reward: u64,
    pub consensus: RwLock<Consensus>,
    pub mempool: Mutex<Vec<Transaction>>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let consensus = Consensus::new();

        // Create the genesis block
        let mut genesis_block = Block::new(String::from("Genesis Block Message"), vec![]);
        genesis_block.hash = consensus.get_hash(&genesis_block);

        Blockchain {
            blocks: RwLock::new(vec![genesis_block]),
            // TODO: We need a way to adjust the reward
            mining_reward: MINER_INITIAL_REWARD,
            consensus: RwLock::new(consensus),
            mempool: Mutex::new(Vec::new()),
        }
    }

    #[allow(dead_code)]
    pub fn length(&self) -> usize {
        self.blocks.read().unwrap().len()
    }

    #[allow(dead_code)]
    pub fn remove_last_block(&self) {
        // TODO: need to return the transactions
    }

    #[allow(dead_code)]
    pub fn add_block(&self, _block: Block) {
        // TODO: Verify the block
    }

    #[allow(dead_code)]
    pub fn compare_hash(&self) {
        // TODO: Return the same length of hash
    }

    #[allow(dead_code)]
    pub fn get_balance(&self, address: &String) -> u64 {
        // TODO: We don't need to check the whole blockchain.
        //       We should only check the last one that the address sent out the money.
        let mut balance: u64 = 0;
        for block in self.blocks.read().unwrap().iter() {
            for transaction in block.transactions.iter() {
                if transaction.receiver == *address {
                    balance += transaction.amount;
                }
                if transaction.sender == *address {
                    balance -= transaction.amount;
                }
            }
        }
        balance
    }

    pub fn mining(&self, miner_address: &str) {
        // Adjust the consensus
        self.consensus
            .write()
            .unwrap()
            .adjust(&self.blocks.read().unwrap());

        // Put the reward of the miner
        let mut transactions: Vec<_> = vec![Transaction {
            sender: String::from(""),
            receiver: miner_address.to_owned(),
            amount: self.mining_reward,
            fee: 0,
            message: String::from("coinbase"),
        }];

        // Put the transaction into it
        // TODO: Check the balance of the sender
        let accept_transaction: Vec<_> = {
            let mut lock = self.mempool.lock().unwrap();
            let size = TRANSACTION_PER_BLOCK.min(lock.len());
            lock.drain(..size).collect()
        };
        transactions.extend(accept_transaction);
        println!("Transactions: {:?}", transactions);
        let mut block = Block::new(
            self.blocks.read().unwrap().last().unwrap().hash.clone(),
            transactions,
        );

        // Calculate the hash
        self.consensus.read().unwrap().calculate(&mut block);

        // Add to the blockchain
        self.blocks.write().unwrap().push(block);
        tracing::info!(
            "New block added: {:?}",
            self.blocks.read().unwrap().last().unwrap()
        );
        tracing::debug!("Blockchain: {:?}", self.blocks);
    }
}
