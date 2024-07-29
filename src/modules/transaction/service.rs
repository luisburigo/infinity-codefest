use std::clone;

use ::redis::Commands;
use axum::http::Error;
use redis::RedisResult;

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

pub fn get_user_transactions(id: String) -> Result<Vec<Transaction>, Error> {
    let mut db = redis_client();
    let tx_list = format!("{}{}", "transactions:".to_owned(), id);

    let res: Vec<String> = db.lrange(tx_list, 0, -1).unwrap();

    let mut transactions: Vec<Transaction> = Vec::new();

    if res.len() == 0 {
        Ok(transactions.clone())
    } else {
        for key in res {
            let transaction: RedisResult<String> = db.get(key);

            match transaction {
                Ok(transaction) => {
                    let parsed_transaction: Transaction =
                        serde_json::from_str(transaction.as_str()).expect("error");

                    transactions.push(parsed_transaction)
                }

                Err(_) => {}
            }
        }

        Ok(transactions.clone())
    }
}

pub fn get_transaction_by_id(id: String) -> Result<Transaction, serde_json::Error> {
    let mut db = redis_client();

    let transaction: String = db.get(id).unwrap();

    let parsed_transaction: Result<Transaction, serde_json::Error> =
        serde_json::from_str(&transaction.as_str());

    parsed_transaction
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
