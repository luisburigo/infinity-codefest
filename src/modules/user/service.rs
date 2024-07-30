extern crate redis;

use axum::Error;
use redis::{Commands, RedisResult};
use uuid::Uuid;

use crate::database::redis::redis_client;
use crate::types::user::types::{User, UserStatus};

pub fn create_user(payload: User) {
    let mut db = redis_client();

    let mut user = payload.clone();
    user.status = Some(UserStatus::Approved);

    let user_id = user.id.unwrap().to_string();
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
                    match serde_json::from_str(user.as_str()) {
                        Ok(parsed_user) => {
                            users.push(parsed_user)
                        }
                        Err(_) => {}
                    }
                }

                Err(_) => {

                }
            }
        }

        Ok(users.clone())
    }
}
