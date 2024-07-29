use axum::extract::Path;
use axum::Json;
use axum::response::IntoResponse;
use chrono::Utc;
use uuid::Uuid;
use crate::types::currency::Currencies;
use crate::types::transaction::types::{TransactionsByType, TransactionIdentifiers, Transaction, TransactionStatus};

pub async fn get_transaction_info(Path((id, tx)): Path<(Uuid, Uuid)>) -> impl IntoResponse {
  let transaction = TransactionIdentifiers {
    id: tx,
    user_id: id,
    sender: Uuid::new_v4(),
  };
  
  Json(transaction)
}

pub async fn list_transactions_by_status(Path((id, status)): Path<(Uuid, TransactionStatus)>) -> impl IntoResponse {
  let transactions = TransactionsByType {
    transactions: vec! [
      Transaction {
        id: Some(Uuid::new_v4()),
        sender: Some(Uuid::new_v4()),
        receiver: Some(Uuid::new_v4()),
        amount: 1000 as f64,
        currency: Some(Currencies::USD),
        // This hash example is wrong, just for test
        hash: Uuid::new_v4().to_string(),
        status: Some(status),
        reason: Some("Initial transaction".to_string()),
        created_at: Utc::now(),
        updated_at: Utc::now(),
      } 
    ],
    user_id: id,
    count: 1
  };
  
  Json(transactions)
}

pub async fn list_transactions() -> impl IntoResponse {
  let transactions = vec![
    Transaction {
      id: Some(Uuid::new_v4()),
      sender: Some(Uuid::new_v4()),
      receiver: Some(Uuid::new_v4()),
      amount: 1000 as f64,
      currency: Some(Currencies::USD),
      // This hash example is wrong, just for test
      hash: Uuid::new_v4().to_string(),
      status: Some(TransactionStatus::Approved),
      reason: Some("Initial transaction".to_string()),
      created_at: Utc::now(),
      updated_at: Utc::now(),
    }
  ];
  
  Json(transactions)
}