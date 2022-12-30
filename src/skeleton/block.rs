// Include libaries
use chrono::prelude::*;
use hex;
use sha2::{Digest, Sha256};
use serde::{Deserialize, Serialize};

use crate::skeleton::transaction::TransactionData;

// Header
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block
{
    pub index: i32,                         // Block Version
    pub previous_hash: String,              // Previous Block Hash
    pub time: i64,                          // Created Time
    pub nonce: u32,                         // Number Used Nonce
    pub vtx: Vec<TransactionData>,          // TXID Vector
    // A nonce is a random or semi-random number that is generated for a specific use.

}

impl Block
{
    // Calculate the hash value of the blocks
    pub fn get_hash(&self) -> String
    {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}{}{}", self.index, self.previous_hash, self.time, self.nonce).as_bytes());
        let hash = hex::encode(hasher.finalize());
        hash
    }

    pub fn serialize(&self) -> Vec<u8>
    {
        let serialized = serde_json::to_string(self).unwrap();
        serialized.into_bytes()
    }

    pub fn new(index: i32, previous_hash: String, vtx: Vec<TransactionData>, nonce: u32) -> Block
    {
        let block = Block {
            index,
            time: Utc::now().timestamp_millis(),
            nonce,
            previous_hash,
            vtx,
        };
        block
    }
}
