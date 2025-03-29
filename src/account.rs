use std::fs;

use base58::ToBase58;
use ripemd::Ripemd160;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountFile {
    name: String,
    address: String,
    public_key_hex: String,
    private_key_hex: String,
}

impl AccountFile {
    fn save_account_to_file(account: &Account) {
        let account_file = AccountFile {
            name: account.name.clone(),
            address: account.address.clone(),
            public_key_hex: hex::encode(account.public_key.serialize()),
            private_key_hex: hex::encode(account.private_key.secret_bytes()),
        };

        let filename = format!("{}.json", account.name);
        let json =
            serde_json::to_string_pretty(&account_file).expect("Unable serialize account to JSON");
        fs::write(&filename, json).expect("Unable to write account to file");

        tracing::info!(
            "Account '{}' is stored into {} successfully!",
            account.name,
            filename
        );
    }

    fn load_account_from_file(name: &String) -> Account {
        let filename = format!("{}.json", name);
        let data = fs::read_to_string(&filename).expect("Unable to read account file");
        let loaded_account: AccountFile =
            serde_json::from_str(&data).expect("Unable to parse account JSON");

        // Extract the public key and private key from the loaded account
        let private_key_bytes =
            hex::decode(loaded_account.private_key_hex).expect("Hex decode failed");
        let public_key_bytes =
            hex::decode(loaded_account.public_key_hex).expect("Hex decode failed");
        let private_key = SecretKey::from_slice(&private_key_bytes).expect("Invalid SecretKey");
        let public_key = PublicKey::from_slice(&public_key_bytes).expect("Invalid PublicKey");

        tracing::info!(
            "Account '{}' is loaded from {} successfully!",
            name,
            filename
        );

        Account {
            name: loaded_account.name,
            address: loaded_account.address,
            public_key,
            private_key,
        }
    }
}

pub struct Account {
    pub name: String,
    pub address: String,
    pub public_key: PublicKey,
    pub private_key: SecretKey,
}

// Calculate SHA-256 => RIPEMD-160
fn hash160(data: &[u8]) -> Vec<u8> {
    let sha256_hash = Sha256::digest(data);
    Ripemd160::digest(sha256_hash).to_vec()
}

// Generate address
fn generate_address(pubkey_hash: Vec<u8>) -> String {
    let mut extended_pubkey_hash = vec![0x00]; // Address prefix
    extended_pubkey_hash.extend(pubkey_hash);

    // Calculate checksum (SHA256 x 2)
    let checksum = &Sha256::digest(Sha256::digest(&extended_pubkey_hash))[0..4];

    // Add checksum
    let mut address_bytes = extended_pubkey_hash.clone();
    address_bytes.extend_from_slice(checksum);

    // Base58 encode
    address_bytes.to_base58()
}

impl Account {
    pub fn create(name: String) -> Account {
        let secp = Secp256k1::new();

        // Generate keys
        let private_key = SecretKey::new(&mut secp256k1::rand::thread_rng());
        let public_key = PublicKey::from_secret_key(&secp, &private_key);

        tracing::debug!("Private Key Hex: {:?}", private_key);
        tracing::debug!("Public Key Hex: {:?}", public_key);

        // Calculate publish key HASH (SHA-256 + RIPEMD-160)
        let pubkey_hash = hash160(&public_key.serialize());

        tracing::debug!(
            "Public key HASH (RIPEMD-160): {:?}",
            hex::encode(&pubkey_hash)
        );

        // Genearte address
        let address = generate_address(pubkey_hash);

        tracing::debug!("Address: {}", address);

        let account = Account {
            name,
            address,
            public_key,
            private_key,
        };

        // Save the account to file
        AccountFile::save_account_to_file(&account);

        account
    }

    pub fn load(name: String) -> Account {
        AccountFile::load_account_from_file(&name)
    }

    // TODO: Able to sign data
}
