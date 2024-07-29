use redis::{aio::MultiplexedConnection, AsyncCommands, Client};
use std::error::Error;

pub async fn redis_client() -> redis::RedisResult<Client> {
    let client = Client::open("redis://127.0.0.1/")?;
    // let connection = client.get_multiplexed_tokio_connection().await?;

    Ok(client)
}

pub struct RedisDb {
    client: redis::Client,
}

impl RedisDb {
    pub fn new(client: redis::Client) -> Self {
        Self { client }
    }

    pub async fn set_key_value(&self, key: &str, value: &str) -> redis::RedisResult<()> {
        let mut con = self.client.get_multiplexed_async_connection().await?;
        con.set(key, value).await?;
        Ok(())
    }
}
