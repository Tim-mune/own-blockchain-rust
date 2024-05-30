use serde::{Serialize,Deserialize};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Transaction{
    id:f64,
    vin:Vec<TxIn>,
    vout:Vec<TxOut>
}


impl Transaction {
    pub fn get_id(&self)->f64 {
        self.id
    }
}


#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct TxOut{
    value:i32,
    pub_key_hash:Vec<u8>
}


#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct TxIn{
    txid:Vec<u8>,
    vout:usize,
    signature:Vec<u8>,
    pub_key:Vec<u8>
}

