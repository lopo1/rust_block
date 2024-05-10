use core::{block, transaction::Transaction};
fn main(){
    let data = "Justin -> Bob 2 btc";
    let transaction = Transaction::new("0x0000", "0x11111", 0,0,data);
    let transactions = vec![transaction];
    let new_block = block::Block::new(transactions,"");

    println!("blcok {:?}",new_block);
    let block_hash = new_block.get_hash();
    println!("block_hash {:?}",block_hash);
}