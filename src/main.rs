mod account;
mod blockchain;
mod connection;
mod consensus;

use clap::{Parser, Subcommand};
use std::sync::{Arc, Mutex};

use account::AccountManager;
use blockchain::{Blockchain, Transaction};
use connection::Connection;
use rand::{rngs::OsRng, TryRngCore};

struct Node {
    connection: Mutex<Connection>,
    _account: String,
    blockchain: Blockchain,
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
            connection: Mutex::new(connection),
            _account: String::from(""),
            blockchain: Blockchain::new(),
            _newpool: Vec::new(),
        }
    }

    async fn listening(&self) {
        loop {
            // TODO: The type should be get_packet_from_queue() -> (packet, address)
            self.connection.lock().unwrap().get_packet_from_queue();
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
            let mut rng = OsRng;
            let new_transaction = Transaction {
                sender: String::from("A"),
                receiver: String::from("B"),
                amount: rng.try_next_u64().unwrap() % 100,
                fee: rng.try_next_u64().unwrap() % 10,
                message: String::from("Hello"),
            };
            // Put the transaction into mempool according to the fee (from large to small)
            {
                let mut mempool = self.blockchain.mempool.lock().unwrap();
                let pos = mempool
                    .binary_search_by(|tx| tx.fee.cmp(&new_transaction.fee).reverse())
                    .unwrap_or_else(|e| e);
                mempool.insert(pos, new_transaction);
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        }
    }

    async fn keep_alive(&self) {
        // TODO: Keepalive implementation
        //   broadcast new_pool
        //   Put them into mempool
    }

    fn mining(&self) {
        loop {
            self.blockchain.mining();
        }
    }
}

/// Rust blockchain POC for wallet management and blockchain interaction
#[derive(Parser)]
#[command(
    version = "0.1",
    author = "CY",
    about = "Rust blockchain POC for wallet management and blockchain interaction"
)]
struct Cli {
    /// IP address to connect to
    #[arg(short, long)]
    connect: Option<String>,

    /// IP address to listen on
    #[arg(short, long)]
    listen: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all wallets
    ListWallet,
    /// Create a new wallet
    CreateWallet {
        /// The name of the wallet
        account: String,
    },
    /// Get the balance of a wallet
    GetBalance {
        /// The account address to check balance
        account: String,
    },
    /// Make a transaction
    Transaction {
        /// The sender's address
        from_addr: String,
        /// The recipient's address
        to_addr: String,
        /// The amount to send
        amount: u64,
    },
    /// Run miner node
    RunMiner { account: String },
    /// Run pure node
    RunNode,
}

#[tokio::main]
async fn main() {
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
    //   2. run Miner
    let args = Cli::parse();

    match args.command {
        Commands::ListWallet => {
            AccountManager::print_accounts();
            return;
        }
        Commands::CreateWallet { account } => {
            let account = AccountManager::create_account(account.to_string());
            println!("Account {} is created.", account.name);
            return;
        }
        Commands::GetBalance { account: _ } => {
            // TODO
        }
        Commands::Transaction {
            from_addr: _,
            to_addr: _,
            amount: _,
        } => {
            // TODO
        }
        Commands::RunMiner { account } => {
            // TODO: Use the account to receive mining money
            let _account = AccountManager::load_account(account);
            // TODO: Initialize the connection and put it into Node
            let _connection = Connection::new();

            let node = Arc::new(Node::new());

            // Listening and Keepalive
            let listening_node = node.clone();
            tokio::spawn(async move { listening_node.listening().await });
            let keep_alive_node = node.clone();
            tokio::spawn(async move { keep_alive_node.keep_alive().await });

            // Mining
            node.mining();
        }
        Commands::RunNode => {
            // TODO
        }
    }
}
