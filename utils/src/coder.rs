use anyhow::Result;
use bincode;
use serde::{Deserialize,Serialize};
use crypto::digest::Digest;
use crypto::sha3::Sha3;
use crate::errors::BlockchainError;
pub fn serialize<T: ?Sized>(value: &T) -> Result<Vec<u8>, BlockchainError> 
    where T: Serialize,
{
    // let seialized = .unwrap();
    Ok(bincode::serialize(value).unwrap())
}

pub fn deserialize<'a,T>(bytes: &'a[u8]) -> Result<T, BlockchainError>
    where T: Deserialize<'a>
{
    let desrialized = bincode::deserialize(bytes).unwrap();
    Ok(desrialized)
}

pub fn get_hash(value: &[u8]) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input(value);
    hasher.result_str()
}

pub fn hash_to_u8(data: &[u8], out: &mut [u8]) {
    let mut hasher = Sha3::sha3_256();
    hasher.input(data);
    hasher.result(out);
}