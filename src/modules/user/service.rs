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

    eprintln!("serialized_data: {:?}", serialized_data);

    match db.set::<String, String, ()>(user_id.clone(), serialized_data) {
        Ok(data) => {
            println!("User Created: {:?}", user_id.clone());
        }
        Err(e) => {
            eprintln!("Error while creating a user: {:?}", e);
        }
    };
}

pub fn get_user(id: Uuid) -> Result<RedisResult<String>, Error> {
    let mut conn = redis_client();

    let user= conn.get(id.to_string());

    Ok(user)
}


pub fn get_all_users() -> Result<Vec<User>, Error> {
    let mut db = redis_client();

    let res: Vec<String> = db.keys("*".to_string()).unwrap();

    let mut users: Vec<User> = Vec::new();

    if res.len() == 0 {
        Ok(users.clone())
    } else {
        for key in res {
            let user: RedisResult<String> = db.get(key);

            match user {
                Ok(user) => {
                    let parsed_user: User = serde_json::from_str(user.as_str()).expect("error");

                    users.push(parsed_user)
                }

                Err(_) => {

                }
            }
        }

        Ok(users.clone())
    }
}
