use axum::{Router, routing::get};

use crate::tasks;

pub fn get_routes() -> Router {
    Router::new()
        .route("/users", get(tasks::user::list_users))
        .route("/users/:id", get(tasks::user::get_user_info))
}
