use crate::handlers::user::ErrorResponse;
use crate::modules::transaction::service::{get_transaction_by_id, get_transactions_by_status};
use crate::types::currency::Currencies;
use crate::types::transaction::types::{
    Transaction, TransactionIdentifiers, TransactionStatus, TransactionsByType,
};

use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use chrono::Utc;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct TransactionInfo {
    id: Uuid,
    user_id: Uuid,
    sender: Uuid,
}

#[derive(Serialize)]
pub struct TransactionsByStatus {
    user_id: String,
    transactions: Vec<Transaction>,
    count: i32
}

pub async fn get_transaction_info(Path((id, tx)): Path<(Uuid, Uuid)>) -> impl IntoResponse {
    let res = get_transaction_by_id(id.to_string(), tx.to_string());

    match res {
        Ok(value) => (
            StatusCode::OK,
            Json(TransactionInfo {
                id: tx,
                user_id: id,
                sender: value.sender.unwrap(),
            })
            .into_response(),
        ),
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                message: "User or transaction not found".to_string(),
            })
            .into_response(),
        ),
    }
}

pub async fn list_transactions_by_status(
    Path((id, status)): Path<(Uuid, TransactionStatus)>,
) -> impl IntoResponse {
    let res = get_transactions_by_status(id.to_string(), status);

    match res {
        Ok(value) => (
            StatusCode::OK,
            Json(TransactionsByStatus {
                count: value.count,
                user_id: value.user_id,
                transactions: value.transactions
            })
              .into_response(),
        ),
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                message: "User or transaction not found".to_string(),
            })
              .into_response(),
        ),
    }
}

pub async fn list_transactions() -> impl IntoResponse {
    let transactions = vec![Transaction {
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
    }];

    Json(transactions)
}
