use crate::{block::Block, storage::Storage, transaction::Transaction};
use std::sync::{Arc, RwLock, atomic::{AtomicUsize, Ordering}};
use tracing::info;
// const CURR_BITS: usize = 8;
pub struct BlockChain<T: Storage> {
    storage: T,
    // pub blocks: Vec<block::Block>,
    tip: Arc<RwLock<String>>,
    pub height: AtomicUsize,
}

impl<T: Storage> BlockChain<T> {
    pub fn new(storage: T) -> Self {
        if let Ok(Some(tip)) = storage.get_tip() {
            let height = storage.get_height().unwrap();
            Self {
                storage,
                tip: Arc::new(RwLock::new(tip)),
                height: AtomicUsize::new(height.unwrap()),
            }
        }else {
            let genesis_block = Block::create_genesis_block();
            let hash = genesis_block.get_hash();
            storage.update_blocks(&hash, &genesis_block, 0 as usize);

            Self {
                storage,
                tip: Arc::new(RwLock::new(hash)),
                height: AtomicUsize::new(0),
            }
        }
    }

    pub fn mine_block(&mut self, transactions: Vec<Transaction>) {

        let mut block = Block::new(transactions, &self.tip.read().unwrap());
        let hash = block.get_hash();
        let  height = self.storage.get_height().unwrap().unwrap_or(0) as i64;
        block.header.height =height+1;
        self.height.fetch_add(1, Ordering::Relaxed);
        self.storage.update_blocks(&hash, &block, self.height.load(Ordering::Relaxed));

        let mut tip = self.tip.write().unwrap();
        *tip = hash;
    }

    // pub fn add_block(&mut self, data: String) {
    //     let pre_block = &self.blocks[self.blocks.len() -1];
    //     let new_block = block::Block::new_block(data,pre_block.hash.clone(),CURR_BITS);
    //     self.blocks.push(new_block);
    //     self.height +=1;
    // }

    // fn new_genesis_block() -> block::Block{
    //     block::Block::new_block("This is genesis block".to_string(), String::from(""),CURR_BITS)
    // }

    // pub fn new_blockchain() -> BlockChain {
    //     BlockChain { 
    //         blocks: vec![BlockChain::new_genesis_block()],
    //         height: 0,
    //     }
    // }

    pub fn blocks_info(&self) {
        let blocks = self.storage.get_block_iter().unwrap();
        for block in blocks {
            info!("{:#?}", block);
        }
    }
}