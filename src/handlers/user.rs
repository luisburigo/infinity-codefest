use axum::extract::Path;
use axum::Json;
use axum::response::IntoResponse;
use chrono::Utc;
use uuid::Uuid;

use crate::modules::user::service::get_user;
use crate::types::currency::Currencies;
use crate::types::transaction::types::{Transaction, TransactionStatus};
use crate::types::user::types::User;

pub async fn list_users() -> impl IntoResponse {
    let users = vec![User {
        id: None,
        name: "Felipe".to_string(),
        email: "".to_string(),
        public_key: "".to_string(),
        status: None,
        balance: None,
        currency: None,
        created_at: Default::default(),
        updated_at: Default::default(),
    }];

    Json(users)
}

pub async fn get_user_info(Path(id): Path<Uuid>) -> impl IntoResponse {
    let res = get_user(id).await.expect("error");
    print!("{}", id);
    // print!("{}", res.expect("error"));

    let x = res.expect("error");

    let json: User = serde_json::from_str(x.as_str()).expect("error");

    Json(json)
}

pub async fn list_user_transactions(Path(id): Path<Uuid>) -> impl IntoResponse {
    let transactions = vec![Transaction {
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
    }];

    Json(transactions)
}
