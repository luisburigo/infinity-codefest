use crate::consumers::transaction::TransactionConsumer;
use crate::consumers::user::UserConsumer;
use crate::modules::user::service::create_user;
use crate::producers::transaction::TransactionProducer;
use crate::producers::user::UserProducer;
use crate::types::transaction::event::TransactionEventMessage;
use crate::types::transaction::types::{ToTransaction, TransactionStatus};
use crate::types::user::event::UserEventMessage;
use crate::types::user::types::{ToUser, UserStatus};
use amiquip::Connection;
use axum::{routing::get, Json, Router};
use modules::transaction::service::create_transaction;
use serde::Serialize;
use uuid::Uuid;

// mods
mod consumers;
mod database;
mod handlers;
mod modules;
mod producers;
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
    // Connect to RabbitMQ
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")
        .expect("Failed to connect to RabbitMQ");

    let user_producer = UserProducer::new(&mut connection);
    let user_consumer = UserConsumer::start(&mut connection).expect("Failed to start UserConsumer");

    let transaction_producer = TransactionProducer::new(&mut connection);
    let transaction_consumer =
        TransactionConsumer::start(&mut connection).expect("Failed to start TransactionConsumer");

    tokio::spawn(async move {
        user_consumer
            .subscribe(move |event| {
                match event {
                    UserEventMessage::Request(payload) => {
                        let mut user = payload.to_user();
                        user.id = Some(Uuid::new_v4());
                        user.status = Option::from(UserStatus::Review);
                        user_producer.publish(user).expect("Pending Error");
                    }
                    UserEventMessage::Pending(payload) => {
                        let mut user = payload.to_user();
                        match user.clone().status.unwrap() {
                            UserStatus::Success => {
                                // println!("User created: {:?}", user.id);
                                user.status = Some(UserStatus::Approved);
                                create_user(user.clone());
                                user_producer.publish(user.clone()).expect("Success Error");
                            }
                            UserStatus::Failed => {
                                // println!("User failed: {:?}", user);
                            }
                            _ => {}
                        }
                    }
                    event => {
                        eprintln!("Unknown event: {:?}", event);
                    }
                }
            })
            .expect("Failed to subscribe UserConsumer");
    });

    tokio::spawn(async move {
        transaction_consumer
            .subscribe(move |event| {
                match event {
                    TransactionEventMessage::Request(payload) => {
                        let mut transaction = payload.to_transaction();
                        transaction.id = Some(Uuid::new_v4());
                        transaction.status = Some(TransactionStatus::Review);
                        transaction_producer
                            .publish(transaction)
                            .expect("Failed to publish transaction");
                    }
                    TransactionEventMessage::Pending(payload) => {
                        let transaction = payload.to_transaction();
                        match transaction.clone().status.unwrap() {
                            TransactionStatus::Success => {
                                // println!("Transaction created: {:?}", transaction.id);

                                create_transaction(transaction.clone())
                                    .expect("Failed to create transaction");
                                transaction_producer
                                    .publish(transaction.clone())
                                    .expect("Failed to publish transaction");
                            }
                            TransactionStatus::Failed => {
                                // println!("Transaction failed: {:?}", transaction.id);
                            }
                            _ => {}
                        }
                    }
                    event => {
                        eprintln!("Unknown event: {:?}", event);
                    }
                }
            })
            .expect("Failed to subscribe TransactionConsumer");
    });

    let app = Router::new()
        .route("/", get(ping))
        .merge(routes::user::get_routes())
        .merge(routes::transaction::get_routes());

    // run our app with hyper, listening on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
