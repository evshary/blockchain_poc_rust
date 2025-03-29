use std::fs;

use base58::ToBase58;
use ripemd::Ripemd160;
use secp256k1::{
    hashes::{sha256, Hash},
    Message, PublicKey, Secp256k1, SecretKey,
};
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
    private_key: SecretKey,
}

impl Account {
    /// Create an account and write it to a file
    pub fn create(name: String) -> Account {
        let secp = Secp256k1::new();

        // Generate keys
        let private_key = SecretKey::new(&mut secp256k1::rand::thread_rng());
        let public_key = PublicKey::from_secret_key(&secp, &private_key);

        tracing::debug!("Private Key Hex: {:?}", private_key);
        tracing::debug!("Public Key Hex: {:?}", public_key);

        // Genearte address
        let address = Account::generate_address(&public_key.serialize());
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

    /// Load an account from a file
    pub fn load(name: String) -> Account {
        AccountFile::load_account_from_file(&name)
    }

    /// Generate a blockchain address from the public key
    ///
    /// Address format: 1 + 20 bytes + 4 bytes
    ///  1 byte: Address prefix (0x00)
    ///  20 bytes: Public key hash (RIPEMD-160)
    ///  4 bytes: Checksum (SHA256 x 2)
    /// Use base58 to encode the address
    ///
    pub fn generate_address(pubkey: &[u8]) -> String {
        // Calculate Public key hash (SHA-256 => RIPEMD-160)
        let sha256_hash = Sha256::digest(pubkey);
        let pubkey_hash = Ripemd160::digest(sha256_hash).to_vec();

        // Address prefix is 0x00
        let mut extended_pubkey_hash = vec![0x00];
        extended_pubkey_hash.extend(pubkey_hash);

        // Calculate checksum (SHA256 x 2)
        let checksum = &Sha256::digest(Sha256::digest(&extended_pubkey_hash))[0..4];

        // Add checksum
        let mut address_bytes = extended_pubkey_hash.clone();
        address_bytes.extend_from_slice(checksum);

        // Base58 encode
        address_bytes.to_base58()
    }

    /// Sign a message with the private key
    #[allow(dead_code)]
    pub fn sign_message(&self, message: &[u8], private_key: &SecretKey) -> Vec<u8> {
        let secp = Secp256k1::new();
        let digest = sha256::Hash::hash(message);
        let message = Message::from_digest(digest.to_byte_array());

        let sig = secp.sign_ecdsa(&message, private_key);
        sig.serialize_compact().to_vec()
    }

    /// Verify a message with the public key
    #[allow(dead_code)]
    pub fn verfiy_signature(public_key: &PublicKey, message: &[u8], signature: &[u8]) -> bool {
        let secp = Secp256k1::new();
        let digest = sha256::Hash::hash(message);
        let message = Message::from_digest(digest.to_byte_array());

        let signature =
            secp256k1::ecdsa::Signature::from_compact(signature).expect("Invalid signature format");
        secp.verify_ecdsa(&message, &signature, public_key).is_ok()
    }
}
