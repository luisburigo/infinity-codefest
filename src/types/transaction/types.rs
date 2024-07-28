use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::types::currency::Currencies;
use crate::types::user::types::UserStatus;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransactionStatus {
  Review,
  Success,
  Failed,
  Approved,
}

#[derive(Debug)]
pub enum TxHash {
  String(String),
  // @TODO: How would be a hash type here?
  // Hash(dyn Hash)
}

#[derive(Debug)]
pub struct Transaction {
  pub id: Option<Uuid>,
  pub sender: Option<Uuid>,
  pub receiver: Option<Uuid>,
  pub amount: i32,
  pub currency: Option<Currencies>,
  pub hash: TxHash,
  pub status: Option<TransactionStatus>,
  pub reason: Option<String>,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}
