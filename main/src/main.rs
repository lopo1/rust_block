use  core::{blockchain, sledbd::SledDb, transaction::Transaction,tx_pool};
use std::env::current_dir;
use std::sync::Arc;

fn main() {
    tracing_subscriber::fmt().init();
    let path = current_dir().unwrap().join("data");
    // println!("path {:?}", path.display());
    let mut bc = blockchain::BlockChain::new(SledDb::new(path));

    let data = "Justin -> Bob 2 btc";
    let value:u64 = 1;
    
    let alice = wallet::wallet::Wallet::new();
    let alice_address = alice.get_address();
    let bob = wallet::wallet::Wallet::new();
    let bob_address = bob.get_address();
    println!("alice address: {}", alice_address);
    println!("private_key_to_hex: {}", alice.private_key_to_hex());
    println!("get_public_key: {:?}", alice.get_public_key());

    let mut  transaction = Transaction::new(&alice_address, &bob_address, value,0,data);

    let signature = utils::secret::sign(&alice.private_key_to_hex(), transaction.hash.as_bytes());
    let signature_str = utils::secret::signature_to_hex(&signature);
    transaction.set_signature(&signature_str);

    let pool = Arc::new(tx_pool::TransactionPool::new());
    pool.add_transaction(transaction).unwrap();
    let txx = pool.get_transactions();
    println!("txx {:?}", txx);
    let txs: Vec<Transaction> = pool.drain_Transaction();
    println!("txs {:?}", txs);
    // let txx = pool.get_transactions();
    // println!("txx {:?}", txx);
    // let transactions = vec![transaction];
    bc.mine_block(txs);

    // let data1 = "Justin -> Bruce 3 btc";
    // let transaction1 = Transaction::new("0x00001", "0x11111", 0,0,Vec::new(),data1);
    // let transactions1 = vec![transaction1];
    // bc.mine_block(transactions1);

    bc.blocks_info();
}
