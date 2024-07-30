
use ::redis::{Commands, RedisResult};
use axum::http::Error;
use serde::Serialize;

use crate::database::redis::redis_client;
use crate::types::transaction::types::{Transaction, TransactionStatus};


#[derive(Serialize)]
pub struct TransactionsByStatus {
    pub user_id: String,
    pub transactions: Vec<Transaction>,
    pub count: i32
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

pub fn get_user_transactions(id: String) -> Result<Vec<Transaction>, Error> {
    let mut db = redis_client();
    let user_tx_list = format!("{}{}", "transactions:".to_owned(), id);

    let res: Vec<String> = db.lrange(user_tx_list, 0, -1).unwrap();

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

pub fn get_transaction_by_id(user_id: String, tx_id: String) -> Result<Transaction, Error> {
    let mut db = redis_client();

    let user_tx_list = format!("{}{}", "transactions:".to_owned(), user_id);

    let res: Vec<String> = db.lrange(user_tx_list, 0, -1).unwrap();

    if res.len() > 0 {
        let haystack: Vec<String> = res.into_iter().collect();
        if haystack.contains(&tx_id) {
            let transaction: String = db.get(tx_id).unwrap();
            let parsed_transaction: Transaction = serde_json::from_str(&transaction.as_str())?;

            Ok(parsed_transaction)
        } else {
            Err()
        }
    } else {
        Err()
    }
}

pub fn get_transactions_by_status(user_id: String, status: TransactionStatus) -> Result<TransactionsByStatus, Error> {
    let mut db = redis_client();

    let user_tx_list = format!("{}{}", "transactions:".to_owned(), user_id);

    let mut res: Vec<String> = db.lrange(user_tx_list, 0, -1).unwrap();

    let mut transactions: Vec<Transaction> = Vec::new();

    if res.len() == 0 {
        Ok(TransactionsByStatus {
            count: res.len() as i32,
            transactions: Vec::new(),
            user_id: user_id.clone(),
        })
    } else {
        for item in res {
            let x = format!("{}{}", "transaction:".to_owned(), item);
            let transaction: RedisResult<String> = db.get(x);

            match transaction {
                Ok(tx) => {
                    let parsed_tx: Transaction = serde_json::from_str(tx.as_str()).expect("error");

                    transactions.push(parsed_tx);
                }

                Err(_) => {}
            }
        }

        Ok(TransactionsByStatus {
            count: res.len() as i32,
            transactions: transactions.clone(),
            user_id: user_id.clone(),
        })
    }
}
