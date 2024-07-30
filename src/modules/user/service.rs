extern crate redis;

use axum::Error;
use redis::{Commands, RedisResult};
use uuid::Uuid;

use crate::database::redis::redis_client;
use crate::types::user::types::User;

pub fn create_user(payload: User) {
    let mut db = redis_client();

    let user_id = payload.id.unwrap().to_string();
    let serialized_data = serde_json::to_string(&payload).unwrap();

    match db.set::<String, String, ()>(user_id.clone(), serialized_data) {
        Ok(data) => {
            println!("User Created: {:?}", user_id.clone());
        }
        Err(e) => {
            eprintln!("Error while creating a user: {:?}", e);
        }
    };
}

pub fn get_user(id: Uuid) -> RedisResult<String> {
    let mut conn = redis_client();

    let user= conn.get(id.to_string());

    user
}
