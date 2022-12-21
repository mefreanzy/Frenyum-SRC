use serde::{Deserialize, Serialize};

// TRANSACTION
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionData
{
    pub from: String,
    pub to: String,
    pub amount: String,
    pub timestamp: i64,
}
