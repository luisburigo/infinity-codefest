extern crate redis;

use axum::Error;
use redis::{Commands, RedisResult};
use uuid::Uuid;

use crate::database::redis::redis_client;
use crate::types::user::types::User;

pub async fn create_user(payload: User) {
    let mut db = redis_client();

    let user_id = serde_json::to_string(&payload.id).unwrap();
    let serialized_data = serde_json::to_string(&payload).unwrap();

    match db.set::<String, String, ()>(user_id, serialized_data) {
        Ok(data) => {
            println!("Change the event here type I guess... ?: {:?}", data);
        }
        Err(e) => {
            eprintln!("Error while creating a user: {:?}", e);
        }
    };
}

pub async fn get_user(id: Uuid) -> Result<RedisResult<String>, Error> {
    let mut conn = redis_client();

    let user: RedisResult<String> = conn.get(id.to_string());
    // let user: User = redis::cmd("GET")
    //   .arg("user")
    //   .query(&mut conn)
    //   .expect("failed to execute GET for 'User'");

    println!("value for 'user' = {:?}", user);

    Ok(user)
}
