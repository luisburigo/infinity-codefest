use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::currency::Currencies;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransactionStatus {
    Review,
    Success,
    Failed,
    Approved,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TxHash {
    String(String),
    // @TODO: How would be a hash type here?
    // Hash(dyn Hash)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub id: Option<Uuid>,
    pub sender: Option<Uuid>,
    pub receiver: Option<Uuid>,
    pub amount: i32,
    pub currency: Option<Currencies>,
    pub hash: String,
    pub status: Option<TransactionStatus>,
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransactionIdentifiers {
    pub user_id: Uuid,
    pub id: Uuid,
    pub sender: Uuid
}

#[derive(Debug, Serialize)]
pub struct TransactionsByType {
    pub user_id: Uuid,
    pub count: i32,
    pub transactions: Vec<Transaction>
}