use axum::{Router, routing::get, response::{IntoResponse}, Json};
use serde::Serialize;

#[derive(Serialize)]
struct User {
    name: String,
}

pub async fn list() -> impl IntoResponse {
    let users = vec![
        User {
          name: "Felipe".to_string()
      }  
    ];
    
    Json(users)
}
pub fn get_routes() -> Router {
    Router::new().route("/users", get(list))
}
