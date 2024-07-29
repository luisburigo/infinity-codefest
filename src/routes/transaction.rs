use axum::{Router, routing::get};

use crate::tasks;

pub fn get_routes() -> Router {
    Router::new().route("/transactions", get(tasks::transaction::list_transactions))
}
