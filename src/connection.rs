pub struct Connection;

// TODO: Create protobuf for command
/*
 * Packet Type:
 * keep_alive: version, length, new_transaction
 * get_hash
 * reply_hash: length, hash list
 * get_blockdata: start number
 * reply_blockdata: length, block data
 * get_balance: address
 * reply get_balance: amount
 * transaction: Transaction struct
 * get_peer
 * reply get_peer: IP list
 */

impl Connection {
    pub fn new() -> Self {
        // TODO: connect the network
        Self {}
    }

    pub fn get_packet_from_queue(&mut self) {
        // TODO: Pop the packets from the queue
    }

    #[allow(dead_code)]
    pub fn send(&self) {
        // TODO: Send the packet
    }

    #[allow(dead_code)]
    pub fn recv(&self) {
        // TODO: Receive the packet
    }

    #[allow(dead_code)]
    pub fn get_peer_list(&self) {
        // TODO: Get the peer list
    }

    #[allow(dead_code)]
    pub fn transfer(&self) {
        // TODO: Transfer the packet
    }
}
