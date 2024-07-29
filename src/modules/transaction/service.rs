use ::redis::Commands;

use crate::database::redis::redis_client;
use crate::types::transaction::types::CreateTransactionPayload;

pub async fn create_transaction(payload: CreateTransactionPayload) {
    let mut db = redis_client();

    let user_id = serde_json::to_string(&payload.user_id).unwrap();
    let serialized_data = serde_json::to_string(&payload).unwrap();

    match db.sadd::<String, String, ()>(user_id, serialized_data) {
        Ok(data) => {
            println!("Change the event here type i guess... ?: {:?}", data)
        }
        Err(e) => {
            eprintln!("Error while creating an transaction: {:?}", e);
        }
    };
}
