use std::error::Error;
use ::redis::Commands;
use redis::{RedisError, RedisResult};
use crate::database::redis::redis_client;
use crate::modules::user::service::{create_user, get_user};
use crate::types::transaction::types::{Transaction, TransactionStatus};
use crate::types::user::types::User;

pub fn create_transaction(mut payload: Transaction) -> Result<Transaction, RedisError> {
    let mut db = redis_client();

    let sender_id = payload.sender.unwrap().to_string();
    let receiver_id = payload.receiver.unwrap().to_string();

    let mut sender = match get_user(payload.sender.unwrap()) {
        Ok(value) => User::from(value),
        Err(e) => return Err(e.into())
    };
    let mut receiver = match get_user(payload.receiver.unwrap()) {
        Ok(value) => User::from(value),
        Err(e) => return Err(e.into())
    };

    // Check sender has balance to send
    let sender_balance = sender.balance.unwrap();
    let sender_currency = sender.currency.clone().unwrap();
    let sender_balance_usd = sender_currency.to_usd(sender_balance);

    let transaction_amount = payload.amount;
    let transaction_currency = payload.currency.clone().unwrap();
    let transaction_amount_usd = transaction_currency.to_usd(transaction_amount);
    let has_balance = sender_balance_usd >= transaction_amount_usd;

    eprintln!("\n----------------------");
    eprintln!("Sending transaction:");
    eprintln!("TX Currency: {:?}", transaction_currency);
    eprintln!("TX Amount: {:?}", transaction_amount);
    eprintln!("TX Amount USD: {:?}", transaction_amount_usd);
    eprintln!("Sender Currency: {:?}", sender_currency);
    eprintln!("Sender Balance: {:?}", sender_balance);
    eprintln!("Sender Balance USD: {:?}", sender_balance_usd);

    match has_balance {
        true => {
            // Update sender balance
            let sender_balance_usd = sender_balance_usd - transaction_amount_usd;
            sender.balance = Some(sender_currency.from_usd(sender_balance_usd));
            create_user(sender.clone());
            eprintln!("Sender New Balance: {:?}", sender.balance);

            // Update receiver balance
            let receiver_balance = receiver.balance.unwrap();
            let receiver_currency = receiver.clone().currency.unwrap();
            let receiver_balance_usd = receiver_currency.to_usd(receiver_balance);

            let receiver_balance_usd = receiver_balance_usd + transaction_amount_usd;
            receiver.balance = Some(receiver_currency.from_usd(receiver_balance_usd));
            create_user(receiver.clone());

            // Update transaction status
            payload.status = Some(TransactionStatus::Approved);

            eprintln!("Receiver Currency: {:?}", receiver_currency);
            eprintln!("Receiver Balance: {:?}", receiver_balance);
            eprintln!("Receiver New Balance: {:?}", receiver.balance);
        }
        false => {
            payload.status = Some(TransactionStatus::Failed);
            eprintln!("Transaction failed: Insufficient balance");
        }
    }

    eprintln!("----------------------\n");

    let sender_key = format!("{}{}", "transactions:".to_owned(), sender_id);
    let receiver_key = format!("{}{}", "transactions:".to_owned(), receiver_id);

    let tx_id = payload.id.unwrap();
    db.rpush::<String, String, ()>(sender_key.clone(), tx_id.to_string()).expect("Failed to add transaction to sender list");
    db.rpush::<String, String, ()>(receiver_key.clone(), tx_id.to_string()).expect("Failed to add transaction to receiver list");

    let serialized_data = serde_json::to_string(&payload).unwrap();
    db.set::<String, String, ()>(tx_id.to_string(), serialized_data).expect("Failed to add transaction to transaction list");

    Ok(payload)
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
