use ::redis::Commands;

use crate::consumers::transaction;
use crate::database::redis::redis_client;
use crate::types::transaction::types::Transaction;

pub fn create_transaction(payload: Transaction) {
    let mut db = redis_client();

    let user_id = payload.sender.unwrap().to_string();
    let serialized_data = serde_json::to_string(&payload).unwrap();

    println!("serialize_data: {:?}", serialized_data);

    match db.sadd::<String, String, ()>(user_id, serialized_data) {
        Ok(data) => {
            println!("Change the event here type i guess... ?: {:?}", data)
        }
        Err(e) => {
            eprintln!("Error while creating an transaction: {:?}", e);
        }
    };
}

// pub fn get_transaction_by_id(transaction_id: String, user_id: String) {
//     let mut db = redis_client();

//     let mut iter: redis::Iter<isize> = redis::cmd("SSCAN")
//         .arg(user_id)
//         .cursor_arg(0)
//         .clone()
//         .iter(&mut db)
//         .unwrap();

//     println!("Iter Result: {:?}", iter)
// }
