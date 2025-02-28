use crate::consensus::Consensus;

pub struct Transaction {
    _sender: String,
    _receiver: String,
    _amount: u64,
    _fee: u64,
    _message: String,
}

pub struct Block {
    pub timestamp: u64,
    pub prev_hash: String,
    pub hash: String,
    pub nonce: u64,
    _transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(prev_hash: String, transactions: Vec<Transaction>) -> Block {
        Block {
            timestamp: 0,
            prev_hash,
            hash: String::from(""),
            nonce: 0,
            _transactions: transactions,
        }
    }
}

pub struct Blockchain {
    pub _blocks: Vec<Block>,
    pub _pending_transactions: Vec<Transaction>,
    pub _mining_reward: u64,
    pub consensus: Consensus,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let consensus = Consensus::new();
        // Create the genesis block
        let mut genesis_block = Block::new(String::from("Genesis Block Message"), vec![]);
        genesis_block.hash = consensus.get_hash(&genesis_block);
        Blockchain {
            _blocks: vec![genesis_block],
            _pending_transactions: Vec::new(),
            _mining_reward: 0,
            consensus,
        }
    }

    pub fn synchronize(&mut self) {
        // TODO: Update the blockchain based on consensus
    }

    pub fn mining(&mut self) {
        // TODO: Create a new block
        // TODO: Put the transaction into it
        // TODO: Calculate the hash
        self.consensus.calculate();
        // TODO: Add to the blockchain
    }
}
