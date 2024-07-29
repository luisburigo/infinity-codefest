use amiquip::Connection;
use axum::{Json, Router, routing::get};
use serde::Serialize;
use uuid::Uuid;
use crate::consumers::user::UserConsumer;
use crate::producers::user::UserProducer;
use crate::types::user::event::{UserEventMessage};
use crate::types::user::types::{ToUser, UserStatus};

// mods
mod database;
mod consumers;
mod handlers;
mod models;
mod modules;
mod routes;
mod types;
mod producers;

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
    // Connect to RabbitMQ
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")
        .expect("Failed to connect to RabbitMQ");

    let user_producer = UserProducer::new(&mut connection);
    let user_consumer = UserConsumer::start(connection).expect("Failed to start UserConsumer");

    user_consumer.subscribe(move |event| {
        match event {
            UserEventMessage::Request(payload) => {
                let mut user = payload.to_user();
                user.id = Some(Uuid::new_v4());
                user.status = Option::from(UserStatus::Review);
                user_producer.publish(user).expect("Pending Error");
            }
            UserEventMessage::Pending(payload) => {
                let user = payload.to_user();
                match user.clone().status.unwrap() {
                    UserStatus::Success => {
                        println!("User created: {:?}", user);
                        user_producer.publish(user.clone()).expect("Success Error");
                    }
                    UserStatus::Failed => {
                        println!("User failed: {:?}", user);
                    }
                    _ => {}
                }
            }
            event => {
                eprintln!("Unknown event: {:?}", event);
            }
        }
    }).expect("Failed to subscribe UserConsumer");

    let app = Router::new()
        .route("/", get(ping))
        .merge(routes::user::get_routes())
        .merge(routes::transaction::get_routes());

    // run our app with hyper, listening on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
