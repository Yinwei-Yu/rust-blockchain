//! 定义区块和区块头结构，处理区块哈希计算和初始化等逻辑

use chrono::prelude::*;
use utils::coder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct BlockHeader {
    /// 区块生成时间
    pub time: i64,
    /// 交易（Merkle）哈希
    pub tx_hash: String,
    /// 上一个区块的哈希
    pub pre_hash: String,
}

#[derive(Debug)]
pub struct Block {
    /// 区块头
    pub header: BlockHeader,
    /// 当前区块哈希
    pub hash: String,
    /// 区块中存放的具体数据
    pub data: String,
}

impl Block {
    /// 设置当前区块哈希
    fn set_hash(&mut self) {
        let header = coder::my_serialize(&(self.header));
        self.hash = coder::get_hash(&header[..]);
    }

    /// 创建并返回一个新的区块
    pub fn new_block(data: String, pre_hash: String) -> Block {
        let transactions = coder::my_serialize(&data);
        let tx_hash = coder::get_hash(&transactions[..]);
        let time = Utc::now().timestamp();
        let mut block = Block {
            header: BlockHeader {
                time,
                tx_hash,
                pre_hash,
            },
            hash: "".to_string(),
            data,
        };
        block.set_hash();
        block
    }
}
