mod blockchain;

use blockchain::{Block, Blockchain};

// TODO: Create protobuf for command

struct Node {
    _ip: String,
    _port: u16,
    _account: String,
    _blockchain: Blockchain,
}

impl Node {
    fn new() -> Node {
        // Create the genesis block
        let mut genesis_block = Block::new(String::from("Genesis Block Message"), vec![]);
        genesis_block.hash = genesis_block.get_hash();
        Node {
            _ip: String::from("127.0.0.1"),
            _port: 8080,
            _account: String::from(""),
            _blockchain: Blockchain {
                _blocks: vec![genesis_block],
                _pending_transactions: Vec::new(),
                _mining_reward: 0,
            },
        }
    }

    fn mining(&self) {
        loop {
            // TODO: Create a new block
            // TODO: Put the transaction into it
            // TODO: Calculate the hash
            // TODO: Add to the blockchain
        }
    }
}

// TODO: struct we need
// Blockchain
// Block
// Transaction
// Connection
//  - connect
//  - send
//  - send(command)
//  - recv(commnad)
//  - peer_list
//  - transfer
//  - get_peer
// Consensus
//  - calculate
//  - verify
//  - adjust difficulties
// Account
// Node
//  - create listener
//    - receive get_peer => get it from Connection => send back
//    - get balance => get it from Blockchain => send back
//    - send_transaction => Verify (Consensus) => transfer
//    - download => get it from Blockchain => send back
//    - get_blockdata => Deal with divergence
//  - send getblockdata periodically
// Miner
//  - do the mining (get data from mempool, calculate, and update blockchain)

fn main() {
    println!("Hello, world!");
    // TODO: Able to parse CLI commands
    // Send command (Need IP:port)
    //   1. get balance
    //   2. send money (need account)
    //   3. get peer list
    // Localhost
    //   1. create account
    //   2. list account
    // Mining (need account)
    //   1. run Node
    //   2. run Minor
    let node = Node::new();
    node.mining();
}
