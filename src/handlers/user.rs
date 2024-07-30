use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;
use uuid::Uuid;

use crate::modules::transaction::service::{get_user_transactions, GetAllTxReturn};
use crate::modules::user::service::{get_all_users, get_user};
use crate::types::transaction::types::{Transaction, TransactionStatus};
use crate::types::user::types::User;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug, Serialize)]
struct ListResponse {
    users: Vec<User>,
    count: i32,
}

pub async fn list_users() -> impl IntoResponse {
    let res = get_all_users();

    match res {
        Ok(value) => {
            (
                StatusCode::OK,
                Json(ListResponse {
                    // @TODO: Implement count
                    users: value,
                    count: 0,
                })
                .into_response(),
            )
        }
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                message: "User not found".to_string(),
            })
            .into_response(),
        ),
    }
}

pub async fn list_user_transactions(Path(id): Path<Uuid>) -> impl IntoResponse {
    let res = get_user_transactions(id.to_string());

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
                message: "User not found".to_string(),
            })
            .into_response(),
        ),
    }
}

pub async fn get_user_info(Path(id): Path<Uuid>) -> impl IntoResponse {
    let res = get_user(id).expect("error");

    match res {
        Ok(value) => {
            let json: User = serde_json::from_str(value.as_str()).expect("error");
            (StatusCode::OK, Json(json).into_response())
        }
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                message: "User not found".to_string(),
            })
            .into_response(),
        ),
    }
}
