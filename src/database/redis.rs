use redis::{ Client};

pub async fn redis_client() -> redis::RedisResult<Client> {
    let client = Client::open("redis://127.0.0.1/")?;
    let connection = client.get_connection().await?;

    Ok(connection)
}
