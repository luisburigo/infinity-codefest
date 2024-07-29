use axum::{routing::get, Json, Router};
use serde::Serialize;

// mods
mod database;
mod consumers;
mod handlers;
mod models;
mod modules;
mod routes;
mod types;

#[derive(Serialize)]
struct PingResponse {
    message: String,
}

async fn ping() -> Json<PingResponse> {
    Json(PingResponse {
        message: "ping".to_string(),
    })
}

#[tokio::main]

async fn main() {
    let app = Router::new()
        .route("/", get(ping))
        .merge(routes::user::get_routes())
        .merge(routes::transaction::get_routes());

    // run our app with hyper, listening on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
