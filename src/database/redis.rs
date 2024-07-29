use redis::{Client, Connection};

pub fn redis_client() -> Connection {
    Client::open("redis://127.0.0.1/")
        .expect("Invalid connection URL")
        .get_connection()
        .expect("Failed to connect to Redis")
}
