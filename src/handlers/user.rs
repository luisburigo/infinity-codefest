use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use chrono::Utc;
use redis::RedisResult;
use serde::Serialize;
use uuid::Uuid;

use crate::modules::user::service::{get_user, get_all_users};
use crate::types::currency::Currencies;
use crate::types::transaction::types::{Transaction, TransactionStatus};
use crate::types::user::types::User;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    message: String,
}

#[derive(Debug, Serialize)]
struct ListResponse {
    users: Vec<User>,
    count: i32
}

pub async fn list_users() -> impl IntoResponse {
    let res = get_all_users();

    match res {
        Ok(value) => {
            (StatusCode::OK, Json(ListResponse {
                // @TODO: Implement count
                users: value,
                count: 0
            }).into_response())
        }
        Err(_) => {
            (StatusCode::NOT_FOUND, Json(ErrorResponse {
                message: "User not found".to_string(),
            }).into_response())
        }
    }
}

pub async fn get_user_info(Path(id): Path<Uuid>) -> impl IntoResponse {
    let res = get_user(id).expect("error");

   match res {
       Ok(value) => {
           let json: User = serde_json::from_str(value.as_str()).expect("error");
           (StatusCode::OK, Json(json).into_response())
       }
       Err(_) => {
           (StatusCode::NOT_FOUND, Json(ErrorResponse {
               message: "User not found".to_string(),
           }).into_response())
       }
   }
}

pub async fn list_user_transactions(Path(id): Path<Uuid>) -> impl IntoResponse {
    let transactions = vec![Transaction {
        id: Some(Uuid::new_v4()),
        sender: Some(id),
        receiver: Some(Uuid::new_v4()),
        amount: 1000.00,
        currency: Some(Currencies::USD),
        // This hash example is wrong, just for test
        hash: Uuid::new_v4().to_string(),
        status: Some(TransactionStatus::Review),
        reason: Some("Initial transaction".to_string()),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }];

    Json(transactions)
}
