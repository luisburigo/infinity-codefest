use axum::extract::{Path, Query};
use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;
// use crate::types::user::

// Temporary fake structure, correct one is above
#[derive(Serialize)]
struct User {
  name: String,
}

pub async fn list_users() -> impl IntoResponse {
  let users = vec![
    User {
      name: "Felipe".to_string()
    }
  ];

  Json(users)
}

pub async fn get_user_info(Path(id): Path<u32>) -> impl IntoResponse {
  print!("{}", id);
  
  let user = User {
    name: id.to_string()
  };
  
  Json(user)
}