use serde::{Deserialize, Serialize};

// TRANSACTION
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionData
{
    pub from: String,   // Sender of transfer.
    pub to: String,     // Recipient of the transfer.
    pub amount: String, // Transfer amount.
    pub timestamp: i64, // Time of transfer.
}
