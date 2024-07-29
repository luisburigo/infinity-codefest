use axum::{Router, routing::get};

use crate::handlers;

pub fn get_routes() -> Router {
    Router::new()
        .route("/users", get(handlers::user::list_users))
        .route("/users/:id", get(handlers::user::get_user_info))
        .route(
            "/users/:id/transactions",
            get(handlers::user::list_user_transactions),
        )
        .route(
            "/users/:id/transactions/:tx",
            get(handlers::transaction::get_transaction_info),
        )
        .route(
            "/users/:id/transactions/status/:status",
            get(handlers::transaction::list_transactions_by_status),
        )
}
