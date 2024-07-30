use ::redis::Commands;
use axum::http::Error;
use serde::Serialize;

use redis::{RedisError, RedisResult};
use sha3::{Digest};
use crate::database::redis::redis_client;
use crate::modules::user::service::{create_user, get_user};
use crate::types::transaction::types::{Transaction, TransactionStatus};
use crate::types::user::types::User;

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

pub fn create_transaction(mut payload: Transaction) -> Result<Transaction, RedisError> {
    let mut db = redis_client();

    let sender_id = payload.sender.unwrap().to_string();
    let receiver_id = payload.receiver.unwrap().to_string();

    let mut sender: Result<User, RedisError> = match get_user(payload.sender.unwrap()) {
        Ok(value) => Ok(User::from(value)),
        Err(e) => Err(e.into())
    };
    let mut receiver: Result<User, RedisError> = match get_user(payload.receiver.unwrap()) {
        Ok(value) => Ok(User::from(value)),
        Err(e) => Err(e.into())
    };

    // Check if has sender and receiver
    if sender.is_ok() && receiver.is_ok() {
        let mut sender = sender.unwrap();
        let mut receiver = receiver.unwrap();

        // Check sender has balance to send
        let sender_balance = sender.balance.unwrap();
        let sender_currency = sender.currency.clone().unwrap();
        let sender_balance_usd = sender_currency.to_usd(sender_balance);

        let transaction_amount = payload.amount;
        let transaction_currency = payload.currency.clone().unwrap();
        let transaction_amount_usd = transaction_currency.to_usd(transaction_amount);
        let has_balance = sender_balance_usd >= transaction_amount_usd;

        // eprintln!("\n----------------------");
        // eprintln!("Sending transaction:");
        // eprintln!("TX Currency: {:?}", transaction_currency);
        // eprintln!("TX Amount: {:?}", transaction_amount);
        // eprintln!("TX Amount USD: {:?}", transaction_amount_usd);
        // eprintln!("Sender Currency: {:?}", sender_currency);
        // eprintln!("Sender Balance: {:?}", sender_balance);
        // eprintln!("Sender Balance USD: {:?}", sender_balance_usd);

        match has_balance {
            true => {
                // Update sender balance
                let sender_balance_usd = sender_balance_usd - transaction_amount_usd;
                sender.balance = Some(sender_currency.from_usd(sender_balance_usd));
                create_user(sender.clone());
                // eprintln!("Sender New Balance: {:?}", sender.balance);

                // Update receiver balance
                let receiver_balance = receiver.balance.unwrap();
                let receiver_currency = receiver.clone().currency.unwrap();
                let receiver_balance_usd = receiver_currency.to_usd(receiver_balance);

                let receiver_balance_usd = receiver_balance_usd + transaction_amount_usd;
                receiver.balance = Some(receiver_currency.from_usd(receiver_balance_usd));
                create_user(receiver.clone());

                // Update transaction status
                payload.status = Some(TransactionStatus::Approved);

                eprintln!("Transaction approved!");
                // eprintln!("Receiver Currency: {:?}", receiver_currency);
                // eprintln!("Receiver Balance: {:?}", receiver_balance);
                // eprintln!("Receiver New Balance: {:?}", receiver.balance);
            }
            false => {
                payload.status = Some(TransactionStatus::Failed);
                payload.reason = Some("Insufficient balance".to_string());
                eprintln!("Transaction failed: Insufficient balance");
            }
        }

        // eprintln!("----------------------\n");
    } else {
        payload.status = Some(TransactionStatus::Failed);

        payload.reason = Some("Sender or receiver not found".to_string());
        eprintln!("Transaction failed: Sender or receiver not found");
    }

    let tx_id = payload.id.unwrap();

    let sender_key = format!("{}{}", "transactions:".to_owned(), sender_id);
    let receiver_key = format!("{}{}", "transactions:".to_owned(), receiver_id);
    let tx_key = format!("{}{}", "transaction:".to_owned(), tx_id);

    db.rpush::<String, String, ()>(sender_key.clone(), tx_id.to_string()).expect("Failed to add transaction to sender list");
    db.rpush::<String, String, ()>(receiver_key.clone(), tx_id.to_string()).expect("Failed to add transaction to receiver list");

    let serialized_data = serde_json::to_string(&payload).unwrap();
    db.set::<String, String, ()>(tx_key.clone(), serialized_data).expect("Failed to add transaction to transaction list");

    Ok(payload)
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

                Err(e) => {
                    eprintln!("Error while getting transaction: {:?}", e);
                }
            }
        }

        Ok(TransactionsByStatus {
            count: transactions.len() as i32,
            transactions: transactions.clone(),
            user_id: user_id.clone(),
        })
    }
}



