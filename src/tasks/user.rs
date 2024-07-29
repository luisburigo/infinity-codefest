use axum::extract::{Path};
use axum::Json;
use axum::response::IntoResponse;
use chrono::Utc;
use serde::Serialize;
use uuid::Uuid;
use crate::types::currency::Currencies;
use crate::types::transaction::types::{Transaction, TransactionStatus};

// Temporary fake structure, correct one is above
#[derive(Serialize)]
struct User {
  name: String,
}

pub async fn list_users() -> impl IntoResponse {
  let users = vec![
    User {
      name: "Felipe".to_string()
    }
  ];

  Json(users)
}

pub async fn get_user_info(Path(id): Path<u32>) -> impl IntoResponse {
  print!("{}", id);
  
  let user = User {
    name: id.to_string()
  };
  
  Json(user)
}

pub async fn list_user_transactions(Path(id): Path<Uuid>) -> impl IntoResponse {
  // @TODO: Check this implementation
  let transactions = vec![
    Transaction {
      id: Some(Uuid::new_v4()),
      sender: Some(id),
      receiver: Some(Uuid::new_v4()),
      amount: 1000,
      currency: Some(Currencies::USD),
      // This hash example is wrong, just for test
      hash: Uuid::new_v4().to_string(),
      status: Some(TransactionStatus::Review),
      reason: Some("Initial transaction".to_string()),
      created_at: Utc::now(),
      updated_at: Utc::now(),    
    },
  ];

  Json(transactions)
}