mod account;
mod blockchain;

use account::Account;
use blockchain::Blockchain;

// TODO: Create protobuf for command

struct Node {
    _ip: String,
    _port: u16,
    _account: String,
    blockchain: Blockchain,
}

impl Node {
    fn new() -> Node {
        Node {
            _ip: String::from("127.0.0.1"),
            _port: 8080,
            _account: String::from(""),
            blockchain: Blockchain::new(),
        }
    }

    fn listening(&mut self) {
        // TODO: Listen from other peers
        //    - receive get_peer => get it from Connection => send back
        //    - get balance => get it from Blockchain => send back
        //    - send_transaction => Verify (Consensus) => transfer
        //    - download => get it from Blockchain => send back
        //    - get_blockdata => Deal with divergence
        self.blockchain.synchronize();
    }

    fn keep_alive(&self) {
        // TODO: Check connection and also sync the blockchain
        //  - send getblockdata periodically
    }

    fn mining(&mut self) {
        loop {
            // TODO: Mining
            //  - do the mining (get data from mempool, calculate, and update blockchain)
            self.blockchain.mining();
        }
    }
}

// TODO: struct we need
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

    // TODO: Create/Read account
    let _account = Account::create("myaccount".to_string());
    let _account = Account::load("myaccount".to_string());
    // Mining
    let mut node = Node::new();
    // TODO: How to run listening, keep_alive and mining
    node.listening();
    node.keep_alive();
    node.mining();
}
