use serde::{Serialize, Deserialize};
use secp256k1::{Secp256k1, SecretKey, PublicKey, Message, ecdsa::RecoverableSignature,All};
use sha3::{Digest, Keccak256};
use sha2::Sha256;
use rand::rngs::OsRng;
use rand::RngCore;
#[derive(Serialize, Deserialize, Clone)]
pub struct Wallet {
    private_key: Vec<u8>,
    public_key: Vec<u8>,
}

impl Wallet {
    pub fn new() -> Self {
        let secp = Secp256k1::<All>::new();

        let mut rng = rand::thread_rng();
        let mut seed = [0u8; 32];
        rng.fill_bytes(&mut seed); // 使用 fill_bytes 填充随机字节
        let private_key = SecretKey::from_slice(&seed).expect("Failed to create private key from seed");
        let public_key = secp256k1::PublicKey::from_secret_key(&secp, &private_key);

        Self {
            private_key: private_key.as_ref().to_vec(),
            public_key: public_key.serialize().to_vec(),
        }
    }

    pub fn from_private_key_hex(private_key_hex: &str) -> Result<Self, hex::FromHexError> {
        let private_key_bytes = hex::decode(private_key_hex)?;
        let public_key = utils::secret::secretkey_to_publick(private_key_hex);

        Ok(Self {
            private_key: private_key_bytes,
            public_key: public_key.serialize().to_vec(),
        })
    }

    pub fn private_key_to_hex(&self) -> String {
        hex::encode(&self.private_key)
    }

    
    pub fn get_address(&self) -> String {
        let pub_key_hash = self.public_key.as_slice();
        let pub_key = PublicKey::from_slice(&pub_key_hash).expect("Failed to create public key from bytes");
        utils::secret::public_key_to_address(&pub_key)
        //////////
        // hex::encode(&self.public_key)
        ///////
        // let secp = Secp256k1::new();
        // let public_key = PublicKey::from_secret_key(&secp, secret_key);
        // self.public_key.so_string()
    }

    pub fn get_private_key(&self) -> &[u8] {
        self.private_key.as_slice()
    }

    pub fn get_public_key(&self) -> &[u8] {
        self.public_key.as_slice()
    }
}

// pub fn public_key_to_address(pub_key: &PublicKey) -> String {
//     let serialized = pub_key.serialize_uncompressed();
//     let hash = Keccak256::digest(&serialized[1..]);  // Remove the 0x04 prefix
//     hex::encode(&hash[12..32])  // Take the last 20 bytes
// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_address() {
        let wallet =Wallet::new();
        println!("private: {}", wallet.private_key_to_hex());
        println!("Address: {}", wallet.get_address());
    }

    #[test]
    fn test_private_to_address() {
        //private: 8ef61c678f7508f8285855d4399adf9add46b42c8e81d98497d97bae26086273
        // Address: 9c8987e2d1f40553260ffd40631a4e41fb3aa32b
        let private = "8ef61c678f7508f8285855d4399adf9add46b42c8e81d98497d97bae26086273";
        let wallet =Wallet::from_private_key_hex(private).unwrap();
        println!("private: {}", wallet.private_key_to_hex());
        println!("Address: {}", wallet.get_address());
    }
}