use crate::handlers::user::ErrorResponse;
use crate::modules::transaction::service::{
    get_all_transactions, get_transaction_by_id, GetAllTxReturn, get_transactions_by_status
};
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
    pub count: i32,
    pub transactions: Vec<Transaction>,
    pub user_id: String,
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
                message: "Transaction not found".to_string(),
            })
            .into_response(),
        ),
    }
}

pub async fn list_transactions() -> impl IntoResponse {
    let res = get_all_transactions();

    match res {
        Ok(value) => (
            StatusCode::OK,
            Json(GetAllTxReturn {
                count: value.count,
                transactions: value.transactions,
            })
            .into_response(),
        ),
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                message: "No transactions founded".to_string(),
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
