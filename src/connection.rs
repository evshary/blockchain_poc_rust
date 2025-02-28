pub struct Connection;

// TODO: Create protobuf for command
// Command Type
// - get_balance: Send from users
// - transaction: Send from users
// - get_version: Keep alive with other nodes, including version and block chain length
// - get_hash: Get the hash list from other nodes
// - reply_hash: Reply the get_hash
// - get_blockdata: Get the block data (different from ours) from other nodes
// - reply_blockdata: Reply the get_blockdata

impl Connection {
    pub fn new() -> Self {
        Self {}
    }

    // TODO: The function we need
    // Connection
    //  - connect
    //  - send
    //  - send(command)
    //  - recv(commnad)
    //  - peer_list
    //  - transfer
    //  - get_peer
}
