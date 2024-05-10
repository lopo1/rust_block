
use serde::{Deserialize,Serialize};
use sha3::{Digest, Keccak256};
use utils::coder;

#[derive(Debug, Serialize, Deserialize,Default,  Clone)]
pub struct Transaction {
    // PublicKey类型需要根据实际使用的加密库来定义
    pub to: String,
    // 使用u64来表示无符号64位整数
    pub value: u64,
    // PublicKey类型需要根据实际使用的加密库来定义
    pub from: String,
    // Signature类型需要根据实际使用的加密库来定义
    pub signature: String,
    // 使用i64来表示有符号64位整数
    pub nonce: i64,
    // Hash类型需要根据实际使用的哈希库来定义
    pub hash: String,
    pub data: Vec<u8>, // transactions data
}


impl Transaction {
    pub fn new(from: &str,to: &str,value:u64,nonce:i64,data: &str) -> Self{
        let mut transaction = Transaction {
            to: to.to_string(),
            value,
            from: from.to_string(),
            signature: "".to_string(),
            nonce,
            data: data.as_bytes().to_vec(),
            ..Default::default()
        };
        transaction.set_hash();
        transaction
    }

    
    pub fn set_hash(&mut self) {
        self.hash = calculate_transaction_hash(&(self));
    }

    pub fn set_signature(&mut self, signature: &str) {
        self.signature = signature.to_string();
    }

    
}


pub fn calculate_transaction_hash(tx: &Transaction) -> String {
    let mut hasher = Keccak256::new();
    // 注意：实际操作中应该序列化交易的所有其他字段，不包括签名
    hasher.update(&tx.to);
    hasher.update(tx.value.to_string());
    hasher.update(&tx.from);
    hasher.update(&tx.data);
    // 根据需要添加更多字段
    hex::encode(hasher.finalize())
}