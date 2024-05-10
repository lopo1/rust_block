use std::sync::{Arc, RwLock};
use crate::transaction::{Transaction,calculate_transaction_hash};
use utils::{secret,coder,errors::BlockchainError};
use std::error::Error;
use std::fmt;



// 定义一个简单的自定义错误类型
#[derive(Debug)]
struct TransactionError {
    message: String,
}

impl fmt::Display for TransactionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for TransactionError {}

#[derive(Default, Debug)]
pub struct TransactionPool {
    transactions: RwLock<Vec<Transaction>>,
}

impl TransactionPool {
    pub fn new() -> Self {
        TransactionPool {
            transactions: RwLock::new(Vec::new()),
        }
    }

    pub fn add_transaction(&self, tx: Transaction) -> Result<(), Box<dyn Error>> {
        let hash = calculate_transaction_hash(&tx);
        if hash!=tx.hash{
            return Err(Box::new(TransactionError { message: "Transaction hash does not match.".to_string() }));
        }
        println!("hash: {:?}", hash);
        println!("tx.signature: {:?}", tx.signature);
        // varify transaction
        if secret::verify_to_str(&tx.from, &tx.hash.as_bytes(),&tx.signature ) == false {
            return Err(Box::new(TransactionError { message: "Transaction signature is invalid.".to_string() }));
        }
        // TODO: 验证余额

        let mut transactions = self.transactions.write().unwrap();
        transactions.push(tx);
        Ok(())
    }

    pub fn get_transactions(&self) -> Vec<Transaction> {
        let transactions = self.transactions.read().unwrap();
        transactions.clone()
    }

    pub fn clear(&self) {
        let mut transactions = self.transactions.write().unwrap();
        transactions.clear();
    }

    // 矿工打包交易的函数，从交易池中取出最多10个交易创建新的区块
    pub fn drain_Transaction(&self) -> Vec<Transaction> {
        // let mut txs = self.transactions.write().unwrap();
        // let block_transactions = txs.drain(..std::cmp::min(txs.len(), 10)).collect();
        // block_transactions

        let txs_to_drain = {
            let txs = self.transactions.read().unwrap();  // First, an immutable borrow to read
            std::cmp::min(txs.len(), 10)  // Decide how many transactions to drain
        };

        let mut txs = self.transactions.write().unwrap();  // Then, a mutable borrow to modify
        txs.drain(..txs_to_drain).collect()  // Perform the draining
    }
}
