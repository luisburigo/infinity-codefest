use axum::{Router, routing::get};

use crate::tasks;

pub fn get_routes() -> Router {
    Router::new()
        .route("/users", get(tasks::user::list_users))
        .route("/users/:id", get(tasks::user::get_user_info))
        .route("/users/:id/transactions", get(tasks::user::list_user_transactions))
        .route("/users/:id/transactions/:tx", get(tasks::transaction::get_transaction_info))
      .route("/users/:id/transactions/status/:status", get(tasks::transaction::list_transactions_by_status))
}
