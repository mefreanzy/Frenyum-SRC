use crate::skeleton::block::Block;
use crate::skeleton::transaction::TransactionData;

use mongodb::{Client, Collection, Database};
use mongodb::bson::doc;
use mongodb::bson;

pub struct Blockchain
{
    client: Client,
    db: Database,
    blocks: Collection,     // Database where blocks are stored.
    txs: Collection,        // Database where transaction are stored.
}

impl Blockchain
{
    pub async fn new(uri: &str, db_name: &str, blockdb_name: &str, txdb_name: &str) -> Result<Blockchain, mongodb::error::Error>
    {
        // Create new client.
        let client = match Client::with_uri_str(uri).await {
            Ok(client) => client,
            Err(e) => return Err(e),
        };

        // Create new databses.
        let db = client.database(db_name);
        let blocks = db.collection(blockdb_name);
        let txs = db.collection(txdb_name);

        // Return blockchain.
        Ok(Blockchain {
            client,
            db,
            blocks,
            txs,
        })
    }

    // Adding a transaction to the database.
    pub async fn add_db_tx(&self, transaction: &TransactionData)
    {
        let bson_transaction = bson::to_document(transaction);

        let bson_transaction = doc! {
            "from": &transaction.from,
            "to": &transaction.to,
            "amount": &transaction.amount,
            "timestamp": &transaction.timestamp,
        };
        self.txs
            .insert_one(bson_transaction, None)
            .await.expect("Failed to insert transaction.");
    }

    // Adding a block to the database.
    pub async fn add_db_block(&self, block: &Block)
    {
        let bson_block = bson::to_document(&block);
        let tx_ids = block.vtx.iter().map(|tx| bson::to_bson(tx).unwrap()).collect::<Vec<_>>();

        let bson_block = doc! {
            "index": &block.index,
            "hash": block.get_hash(),
            "previous_hash": &block.previous_hash,
            "time": &block.time,
            "nonce": &block.nonce,
            "tx_ids": tx_ids,
        };
        self.blocks
            .insert_one(bson_block, None)
            .await.expect("Failed to insert block.");
    }
}
