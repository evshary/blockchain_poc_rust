// TODO: Create protobuf for command

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
// Minor
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
}
