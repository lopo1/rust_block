use std::path::Path;
use sled::{Db, IVec, transaction::TransactionResult};
use crate::block::Block;
use utils::coder::{serialize, deserialize};
use utils::errors::BlockchainError;
use std::result;
use crate::storage::{Storage,TIP_KEY, TABLE_OF_BLOCK, HEIGHT, StorageIterator};

pub struct SledDb {
    db: Db
}

impl SledDb {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            db: sled::open(path).unwrap()
        }
    }

    fn get_full_key(table: &str, key: &str) -> String {
        format!("{}:{}", table, key)
    }
}

impl Storage for SledDb {
    fn get_tip(&self) -> Result<Option<String>, BlockchainError> {
        let result = self.db.get(TIP_KEY)?.map(|v| deserialize::<String>(&v.to_vec()));
        result.map_or(Ok(None), |v| v.map(Some))
    }

    fn get_block(&self, key: &str) -> Result<Option<Block>, BlockchainError> {
        let name = Self::get_full_key(TABLE_OF_BLOCK, key);
        let result = self.db.get(name)?.map(|v| MyIVec::from(v).into());
        Ok(result)
    }

    fn get_height(&self) -> Result<Option<usize>, BlockchainError> {
        let result = self.db.get(HEIGHT)?.map(|v| deserialize::<usize>(&v.to_vec()));
        result.map_or(Ok(None), |v| v.map(Some))
    }

    fn update_blocks(&self, key: &str, block: &Block, height: usize) {
        let _: TransactionResult<(), ()> = self.db.transaction(|db| {
            let name = Self::get_full_key(TABLE_OF_BLOCK, key);
            db.insert(name.as_str(), serialize(block).unwrap())?;
            db.insert(TIP_KEY, serialize(key).unwrap())?;
            db.insert(HEIGHT, serialize(&height).unwrap())?;
            db.flush();
            Ok(())
        });
    }

    fn get_block_iter(&self) -> Result<Box<dyn Iterator<Item = Block>>, BlockchainError> {
        let prefix = format!("{}:", TABLE_OF_BLOCK);
        // let iter = StorageIterator::new(self.db.scan_prefix(prefix));
        let iter = self.db.scan_prefix(prefix)
        .map(|result| result.map_or_else(
            |_e| Block::default(), // 错误处理
            |(_, v)| Block::from(MyIVec::from(v)) // 正确的类型转换
        ));
        Ok(Box::new(iter))
    }
}

pub struct ResultBlock(result::Result<(IVec, IVec), sled::Error>);
// 定义一个新的结构体包裹IVec
pub struct MyIVec(IVec);

// 实现从IVec到MyIVec的转换
impl From<IVec> for MyIVec {
    fn from(v: IVec) -> Self {
        MyIVec(v)
    }
}

// 实现从MyIVec到Block的转换
impl From<MyIVec> for Block {
    fn from(my_ivec: MyIVec) -> Self {
        let result = bincode::deserialize::<Block>(&my_ivec.0);
        match result {
            Ok(block) => block,
            Err(_) => Block::default(),
        }
    }
}



// impl From<IVec> for Block {
//     fn from(v: IVec) -> Self {
//         let result = deserialize::<Block>(&v.to_vec());
//         match result {
//             Ok(block) => block,
//             Err(_) => Block::default(),
//         }
//     }
// }

// impl From<Result<(IVec, IVec), sled::Error>> for Block {
//     fn from(result: Result<(IVec, IVec), sled::Error>) -> Self {
//         match result {
//             Ok((_, v)) => match deserialize::<Block>(&v.to_vec()) {
//                     Ok(block) => block,
//                     Err(_) => Block::default(),
//             },
//             Err(_) => Block::default(),
//         }
//     }
// }

impl From<ResultBlock> for Block {
    fn from(rb: ResultBlock) -> Self {
        match rb.0 {
            Ok((_, v)) => {
                let result = bincode::deserialize::<Block>(&v);
                match result {
                    Ok(block) => block,
                    Err(_) => Block::default(),
                }
            },
            Err(_) => Block::default(),
        }
    }
}