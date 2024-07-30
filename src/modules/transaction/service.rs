use ::redis::Commands;
use axum::http::Error;
use redis::RedisResult;
use serde::Serialize;

use crate::database::redis::redis_client;
use crate::types::transaction::types::Transaction;

#[derive(Serialize)]
pub struct GetAllTxReturn {
    pub transactions: Vec<Transaction>,
    pub count: usize,
}

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

pub fn get_all_transactions() -> Result<GetAllTxReturn, Error> {
    let mut db = redis_client();
    let formatted_tx_key = format!("{}{}", "transaction:".to_owned(), '*');

    let res: Vec<String> = db.keys(formatted_tx_key).unwrap();

    let mut transactions: Vec<Transaction> = Vec::new();

    let count = res.len();

    if res.len() == 0 {
        Ok(GetAllTxReturn {
            transactions: transactions.clone(),
            count,
        })
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

        Ok(GetAllTxReturn {
            transactions: transactions.clone(),
            count,
        })
    }
}

pub fn get_user_transactions(id: String) -> Result<GetAllTxReturn, Error> {
    let mut db = redis_client();
    let tx_list = format!("{}{}", "transactions:".to_owned(), id);

    let res: Vec<String> = db.lrange(tx_list, 0, -1).unwrap();

    let mut transactions: Vec<Transaction> = Vec::new();

    let count = res.len();

    if res.len() == 0 {
        Ok(GetAllTxReturn {
            transactions: transactions.clone(),
            count,
        })
    } else {
        for key in res {
            let formatted_tx_id = format!("{}{}", "transaction:".to_owned(), key);
            let transaction: RedisResult<String> = db.get(formatted_tx_id);

            match transaction {
                Ok(transaction) => {
                    let parsed_transaction: Transaction =
                        serde_json::from_str(transaction.as_str()).expect("error");

                    transactions.push(parsed_transaction)
                }

                Err(_) => {}
            }
        }

        Ok(GetAllTxReturn {
            transactions: transactions.clone(),
            count,
        })
    }
}

pub fn get_transaction_by_id(id: String) -> Result<Transaction, serde_json::Error> {
    let mut db = redis_client();
    let formatted_tx_id = format!("{}{}", "transaction:".to_owned(), id);

    let transaction: String = db.get(formatted_tx_id).unwrap();

    let parsed_transaction: Result<Transaction, serde_json::Error> =
        serde_json::from_str(&transaction.as_str());

    parsed_transaction
}
