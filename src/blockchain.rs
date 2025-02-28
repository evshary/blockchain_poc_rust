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

    pub fn get_hash(&self) -> String {
        let _hash_input = format!("{}{}{}", self.timestamp, self.prev_hash, self.nonce,);
        // TODO: Covert Block into hash
        String::new()
    }
}

pub struct Blockchain {
    pub _blocks: Vec<Block>,
    pub _pending_transactions: Vec<Transaction>,
    pub _mining_reward: u64,
}
