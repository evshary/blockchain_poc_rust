mod account;
mod blockchain;
mod connection;
mod consensus;

use account::Account;
use blockchain::{Blockchain, Transaction};
use connection::Connection;

struct Node {
    connection: Connection,
    _account: String,
    blockchain: Blockchain,
    _mempool: Vec<Transaction>,
    _newpool: Vec<Transaction>,
}

impl Node {
    fn new() -> Node {
        tracing::debug!("Initialize the Node");

        // TODO: Initialize the connection
        let _ip = String::from("127.0.0.1");
        let _port = 8080;
        let connection = Connection::new();

        Node {
            connection,
            _account: String::from(""),
            blockchain: Blockchain::new(),
            _mempool: Vec::new(),
            _newpool: Vec::new(),
        }
    }

    fn listening(&mut self) {
        // TODO: The type should be get_packet_from_queue() -> (packet, address)
        self.connection.get_packet_from_queue();
        /* TODO:
          Potential type:
          1. keep alive
            get the transaction and put it into mempool
            compare the length of chain => get_hash if necessary
          2. get_hash
            Get the hash from Blockchain => send back
          3. reply_hash
            Compare the hash difference
          4. get_blockdata
            Get the required block from Blockchain => send back
          5. reply_blockdata
            Lock the Blockchain
            for old block:
              push back to mempool
            for new block:
              verify the blockdata
              remove the one in mempool
              update the Blockchain
          6. get_peer
            get it from Connection => send back
          7. get balance
            get it from Blockchain => send back
          8. send_transaction
            Verify (Consensus)
            broadcast
            put it into newpool
        */
    }

    fn keep_alive(&self) {
        // TODO: Keepalive implementation
        //   broadcast new_pool
        //   Put them into mempool
    }

    fn mining(&mut self) {
        loop {
            // TODO: Mining
            //   do the mining (get data from mempool, calculate, and update blockchain)
            // TODO: Need a way to lock blockchain if we want to update it while receiving longer chain
            self.blockchain.mining();
        }
    }
}

fn main() {
    // Initialize the logger
    tracing_subscriber::fmt::init();

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
    // TODO: Initialize the connection and put it into Node
    let _connection = Connection::new();
    // Mining
    let mut node = Node::new();
    // TODO: How to run listening, keep_alive and mining
    node.listening();
    node.keep_alive();
    node.mining();
}
