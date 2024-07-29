use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::user::{Currencies, User};

#[derive(Serialize, Deserialize)]
pub enum TransactionStatus {
    REVIEW,
    SUCCESS,
    FAILED,
    APPROVED,
}

impl TransactionStatus {
    pub fn enum_to_string(&self) -> &str {
        match self {
            TransactionStatus::REVIEW => "review",
            TransactionStatus::SUCCESS => "success",
            TransactionStatus::FAILED => "failed",
            TransactionStatus::APPROVED => "approved",
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    sender: User,
    receiver: User,
    amount: f64,
    currency: Currencies,
    hash: String,
    status: TransactionStatus,
    reason: Option<String>,
}
