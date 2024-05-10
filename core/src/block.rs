use serde::{Deserialize,Serialize};
use chrono::prelude::*;
use utils::coder;
use crate::transaction::Transaction;
#[derive(Serialize,Deserialize,Debug,PartialEq,Eq,Clone, Default)]
pub struct BlockHeader{
    pub time: i64,
    pub tx_hash: String,
    pub pre_hash: String,
    pub height: i64,
    pub nonce: usize,
}

impl BlockHeader {
    fn new(pre_hash: &str,tx_hash:&str) -> Self {
        Self {
            time: Utc::now().timestamp(),
            tx_hash: tx_hash.into(),
            pre_hash: pre_hash.into(),
            height: 0,
            nonce: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Block {
    pub header: BlockHeader,
    pub hash: String,
    pub transactions: Vec<Transaction>,
    pub validator:String,
	pub signature: String, // Signature
    // pub data: String, // transactions data
}

impl Block {
    pub fn set_hash(&mut self) {
        let header = coder::serialize(&(self.header)).unwrap();
        self.hash = coder::get_hash(&header[..]);
    }

    pub fn create_genesis_block() -> Self{
         let data = "This is genesis block";
         let transaction = Transaction::new("0x0000", "0x11111", 0,0,data);
         let transactions = vec![transaction];
        Self::new(transactions, "")
    }

    pub fn new(transactions: Vec<Transaction>,pre_hash: &str) -> Self{
        let serialized_transactions = coder::serialize(&transactions).unwrap();
        let tx_hash = coder::get_hash(&serialized_transactions[..]);
        let mut block = Block {
            header: BlockHeader::new(pre_hash, &tx_hash),
            hash: "".to_string(),
            transactions: transactions,
            ..Default::default()
        };
        block.set_hash();

        
        block
    }

    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }

    

    pub fn set_nonce(&mut self, nonce: usize) {
        self.header.nonce = nonce;
    }

    pub fn get_header(&self) -> BlockHeader {
        self.header.clone()
    }


}