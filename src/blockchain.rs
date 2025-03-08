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
    pub blocks: Vec<Block>,
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
            blocks: vec![genesis_block],
            _mining_reward: 0,
            consensus,
        }
    }

    #[allow(dead_code)]
    pub fn length(&self) -> usize {
        self.blocks.len()
    }

    #[allow(dead_code)]
    pub fn remove_last_block(&mut self) {
        // TODO: need to return the transactions
    }

    #[allow(dead_code)]
    pub fn add_block(&mut self, _block: Block) {
        // TODO: Verify the block
    }

    #[allow(dead_code)]
    pub fn compare_hash(&self) {
        // TODO: Return the same length of hash
    }

    pub fn mining(&mut self) {
        // TODO: Create a new block
        // TODO: Put the transaction into it
        // TODO: Calculate the hash
        self.consensus.calculate();
        // TODO: Add to the blockchain
    }
}
