extern crate redis;

use std::io::Error;

use redis::{Commands, RedisResult};
use uuid::Uuid;

use crate::database::redis::redis_client;
use crate::types::user::types::User;

//
pub async fn create_user(payload: User) {
    let mut db = redis_client();

    let user_id = payload.id.unwrap();
    let serialized_data = match serde_json::to_string(&payload) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error serializing user data: {:?}", e);
            return;
        }
    };

    let res: () = db
        .set(&user_id.to_string(), &serialized_data)
        .expect("error");

    println!("{:?}", res)
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
