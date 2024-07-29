use ::redis::Commands;

use crate::database::redis::{self, redis_client};
use crate::types::user::types::User;

pub async fn create_user(payload: User) {
    // let db = redis_client().await?;
    //
    // let user_id = payload.id.to_string();
    // let serialized_data = match serde_json::to_string(&payload) {
    //     Ok(data) => data,
    //     Err(e) => {
    //         eprintln!("Error serializing user data: {:?}", e);
    //         return;
    //     }
    // };
    //
    // match db.set(&user_id, &serialized_data).await {
    //     Ok(data) => {
    //         println!("Change the event here type i guess... ?: {:?}", data)
    //     }
    //     Err(e) => {
    //         eprintln!("Error while creating an user: {:?}", e);
    //     }
    // };
}
