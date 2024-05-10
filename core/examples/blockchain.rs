use  core::{blockchain, sledbd::SledDb, transaction::Transaction};
use std::env::current_dir;

fn main(){
    tracing_subscriber::fmt().init();
    let path = current_dir().unwrap().join("test_data");
    let mut bc = blockchain::BlockChain::new(SledDb::new(path));

    let data = "Justin -> Bob 2 btc";
    let transaction = Transaction::new("0x0000", "0x11111", 0,0,data);
    let transactions = vec![transaction];
    bc.mine_block(transactions);

    let data1 = "Justin -> Bruce 3 btc";
    let transaction1 = Transaction::new("0x00001", "0x11111", 0,0,data1);
    let transactions1 = vec![transaction1];
    bc.mine_block(transactions1);

    bc.blocks_info();
}
