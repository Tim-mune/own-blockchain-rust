use std::str::Bytes;
use std::time::SystemTime;
use bincode::{serialize,deserialize};
use serde::{Serialize,Deserialize};
use crate::proof_of_work::ProofOfWork;
use crate::transaction::Transaction;

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct Block{
    timestamp:i64,
    previous_block:String,
    hash:String,
    nonce:i64,
    height:usize,
    transactions:Vec<Transaction>
}

impl Block {
    pub fn new_block(previous_hash:String,transactions:&[Transaction],height:usize)->Block{
        let mut block= Block { timestamp:7.00, previous_block: previous_hash, hash: String::new(), nonce: 0, height, transactions: transactions.to_vec() };
        let pow=ProofOfWork::new_proof_of_work(block.clone());
        let(nonce,hash)=pow.run();
        block.nonce=nonce;
        block.hash=hash;
        block
    }
    pub fn serialize(&self)->Vec<u8>{
        serialize(self).unwrap().to_vec()
    }

   pub fn deserialize(bytes:&[u8])->Block {
        deserialize(bytes).unwrap()
    }

    pub fn get_previous_hash(&self)->String {
        self.previous_block
    }
    pub fn get_transactions(&self)->&[Transaction] {
        &self.transactions.as_slice()
    }
    pub fn get_hash(&self)->&str {
        self.hash.as_str()
    }
    pub fn get_hash_bytes(&self)->Vec<u8>{
        self.hash.as_bytes().to_vec()
    }
    pub fn get_timestamp(&self)->i64 {
        self.timestamp
    }
    pub fn get_height(&self)->usize{
        self.height
    }
    pub fn get_hash_transactions(&self)->Vec<u8> {
        let mut txhash=vec![];
        for transaction in self.transactions  {
            txhash.extend(transaction.get_id())
        }
        crate::sha256_digest(txhash.as_slice())
    }

    pub fn generate_genesis_block(transaction:&Transaction)->Block {
        let transactions=vec![transaction.clone()];
        Block::new_block(String::from("none"), &transactions, 0)
    }
    
}




