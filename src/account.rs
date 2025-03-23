use base58::ToBase58;
use ripemd::Ripemd160;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use sha2::{Digest, Sha256};

pub struct Account {
    _name: String,
    _pub_key: String,
    _priv_key: String,
}

// Calculate SHA-256 => RIPEMD-160
fn hash160(data: &[u8]) -> Vec<u8> {
    let sha256_hash = Sha256::digest(data);
    Ripemd160::digest(sha256_hash).to_vec()
}

// Generate BTC address
fn generate_btc_address(pubkey_hash: Vec<u8>) -> String {
    let mut extended_pubkey_hash = vec![0x00]; // Bitcoin address prefix
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
    // TODO: Able to write the account data to the file
    pub fn create(name: String) -> Account {
        let secp = Secp256k1::new();

        // Generate keys
        let secret_key = SecretKey::new(&mut secp256k1::rand::thread_rng());
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);

        tracing::debug!("Private key: {:?}", secret_key);
        tracing::debug!("Public key: {:?}", public_key);

        // Calculate publish key HASH (SHA-256 + RIPEMD-160)
        let pubkey_hash = hash160(&public_key.serialize());

        tracing::debug!(
            "Public key HASH (RIPEMD-160): {:?}",
            hex::encode(&pubkey_hash)
        );

        // Genearte BTC address
        let address = generate_btc_address(pubkey_hash);

        tracing::debug!("BTC Address: {}", address);

        Account {
            _name: name,
            _pub_key: String::new(),
            _priv_key: String::new(),
        }
    }

    // TODO: Able to read the account data from the file
    pub fn load(name: String) -> Account {
        Account {
            _name: name,
            _pub_key: String::new(),
            _priv_key: String::new(),
        }
    }

    // TODO: Able to sign data
}
