use secp256k1::{Secp256k1, SecretKey, PublicKey, Message, ecdsa::RecoverableSignature,ecdsa::RecoveryId};
use sha3::{Digest, Keccak256};
use sha2::Sha256;


pub fn sign( private_key:&str,message: &[u8]) -> RecoverableSignature {
    let secp = Secp256k1::new();
    let message_hash = Sha256::digest(message);
    let message = Message::from_digest_slice(&message_hash).expect("Message could be created");
    secp.sign_ecdsa_recoverable(&message, &(private_by_str(private_key)))
}
pub fn verify( public_key: &PublicKey, message: &[u8], signature: &RecoverableSignature) -> bool {
    match recover_public_key(message, signature) {
        Ok(recovered_key) => recovered_key == *public_key,
        Err(_) => false,
    }
}
pub fn recover_public_key( message: &[u8], signature: &RecoverableSignature) -> Result<PublicKey, secp256k1::Error> {
    let secp = Secp256k1::new();
    let message_hash = Sha256::digest(message);
    let message = Message::from_digest_slice(&message_hash).expect("Message could be created");
    secp.recover_ecdsa(&message, signature)
}

pub fn public_key_to_address(pub_key: &PublicKey) -> String {
    let serialized = pub_key.serialize_uncompressed();
    let hash = Keccak256::digest(&serialized[1..]);  // Remove the 0x04 prefix
    hex::encode(&hash[12..32])  // Take the last 20 bytes
}
pub fn private_by_str(private_key_hex: &str) -> SecretKey {
    let private_key_bytes = hex::decode(private_key_hex).unwrap();
    SecretKey::from_slice(&private_key_bytes).expect("Invalid private key")
}

pub fn secretkey_to_publick(private_key: &str) -> PublicKey {
    let secp = Secp256k1::new();
    secp256k1::PublicKey::from_secret_key(&secp, &(private_by_str(private_key)))
}

pub fn public_key_by_str(public_key_hex: &str) -> PublicKey {
    println!("public_key_by_str: {}", public_key_hex);
    let publ = hex::decode(&public_key_hex).unwrap();
    PublicKey::from_slice(&publ).expect("Failed to create public key from bytes")
    
}

pub fn verify_to_str( from: &str, message: &[u8], signature: &str) -> bool {
    let sig = hex_to_recoverable_signature(signature).unwrap();
    let recovered_key = recover_public_key(message, &sig).unwrap();
    let recover_address = public_key_to_address(&recovered_key);
    return recover_address == from;
    
}

pub fn hex_to_recoverable_signature(hex_str: &str) -> Result<RecoverableSignature, secp256k1::Error> {
    // 将十六进制字符串解码为字节
    let bytes = hex::decode(hex_str).map_err(|_| secp256k1::Error::InvalidSignature)?;

    // 检查解码后的字节长度是否正确
    if bytes.len() != 65 {
        return Err(secp256k1::Error::InvalidSignature);
    }

    // 最后一个字节是恢复ID
    let recovery_id = RecoveryId::from_i32(bytes[64] as i32)?;
    
    // 前64个字节是签名数据
    RecoverableSignature::from_compact(&bytes[0..64], recovery_id)
}

pub fn signature_to_hex(signature: &RecoverableSignature) -> String {
    let (rec_id, data) = signature.serialize_compact();
    let mut data_vec = vec![0u8; 65]; // 65 bytes: 64 bytes of data + 1 byte of rec_id
    data_vec[0..64].copy_from_slice(&data);
    data_vec[64] = rec_id.to_i32() as u8; // 添加恢复ID作为最后一个字节
    hex::encode(data_vec)
}