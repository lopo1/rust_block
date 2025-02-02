use utils::errors::BlockchainError;
use crate::block::Block;



pub use crate::sledbd::SledDb;

pub const TIP_KEY: &str = "tip_hash";
pub const HEIGHT: &str = "height";
pub const TABLE_OF_BLOCK: &str = "blocks";

pub trait Storage: Send + Sync + 'static {
    fn get_tip(&self) -> Result<Option<String>, BlockchainError>;
    fn get_block(&self, key: &str) -> Result<Option<Block>, BlockchainError>;
    fn get_height(&self) -> Result<Option<usize>, BlockchainError>;
    fn update_blocks(&self, key: &str, block: &Block, height: usize);
    fn get_block_iter(&self) -> Result<Box<dyn Iterator<Item = Block>>, BlockchainError>;
}

pub struct StorageIterator<T> {
    data: T
}

impl<T> StorageIterator<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

impl<T> Iterator for StorageIterator<T> 
where
    T: Iterator,
    T::Item: Into<Block>
{
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        self.data.next().map(|v| v.into())
    }
}