use redis::{Client, Connection};

pub fn redis_client() -> Connection {
    // let client = Client::open("redis://127.0.0.1/").expect("Error opening URL connection");
    // let connection = client.get_connection().expect("Error connecting to redis");

    // Ok(connection)

    Client::open("redis://127.0.0.1/")
        .expect("Invalid connection URL")
        .get_connection()
        .expect("Failed to connect to Redis")
}
