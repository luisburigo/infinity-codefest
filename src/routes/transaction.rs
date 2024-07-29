use axum::{Router, routing::get};

use crate::handlers;

pub fn get_routes() -> Router {
    Router::new().route(
        "/transactions",
        get(handlers::transaction::list_transactions),
    )
}
