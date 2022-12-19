use serde::{Deserialize, Serialize};

// TRANSACTION
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionData
{
    pub sender: String,
    pub recevier: String,
    pub amount: String,
    pub timestamp: i64,
}
