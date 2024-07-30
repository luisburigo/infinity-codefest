use std::cmp::PartialEq;
use ::redis::Commands;
use axum::http::Error;
use redis::RedisResult;
use serde::Serialize;

use crate::database::redis::redis_client;
use crate::types::transaction::types::{Transaction, TransactionStatus};

#[derive(Serialize)]
pub struct GetAllTxReturn {
    pub transactions: Vec<Transaction>,
    pub count: usize,
}

#[derive(Serialize)]
pub struct TransactionsByStatus {
    pub count: i32,
    pub transactions: Vec<Transaction>,
    pub user_id: String,
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

pub fn get_transaction_by_id(user_id: String, tx_id: String) -> Result<Transaction, axum::Error> {
    let mut db = redis_client();

    let user_tx_list = format!("{}{}", "transactions:".to_owned(), user_id);

    let res: Vec<String> = db.lrange(user_tx_list, 0, -1).unwrap();

    if res.len() > 0 {
        let haystack: Vec<String> = res.into_iter().collect();
        if haystack.contains(&tx_id) {
            let transaction: String = db.get(tx_id).unwrap();
            let parsed_transaction: Transaction = serde_json::from_str(&transaction.as_str()).expect("error");

            Ok(parsed_transaction)
        } else {
            Err(axum::Error::new("User or transaction not found"))
        }
    } else {
        Err(axum::Error::new("User or transaction not found"))
    }
}

impl PartialEq for TransactionStatus {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TransactionStatus::Success, TransactionStatus::Success) => true,
            (TransactionStatus::Review, TransactionStatus::Review) => true,
            (TransactionStatus::Failed, TransactionStatus::Failed) => true,
            (TransactionStatus::Approved, TransactionStatus::Approved) => true,
            _ => false
        }
    }
}

pub fn get_transactions_by_status(user_id: String, status: TransactionStatus) -> Result<TransactionsByStatus, Error> {
    let mut db = redis_client();

    let user_tx_list = format!("{}{}", "transactions:".to_owned(), user_id);

    let res: Vec<String> = db.lrange(user_tx_list, 0, -1).unwrap();

    let mut transactions: Vec<Transaction> = Vec::new();

    let count = res.len();

    if count == 0 {
        Ok(TransactionsByStatus {
            count: count as i32,
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

                    let is_same_status= match parsed_tx.clone().status {
                        Some(parsed_tx_status) => parsed_tx_status == status,
                        _ => false
                    };

                    if is_same_status {
                        transactions.push(parsed_tx.clone())
                    }
                }

                Err(_) => {}
            }
        }

        Ok(TransactionsByStatus {
            count: count as i32,
            transactions: transactions.clone(),
            user_id: user_id.clone(),
        })
    }
}


