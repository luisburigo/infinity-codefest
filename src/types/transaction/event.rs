use crate::types::currency::Currencies;
use crate::types::transaction::types::{ToTransaction, Transaction, TransactionStatus};
use chrono::{DateTime, Utc};
use serde::de::{Unexpected, Visitor};
use serde::{de, Deserialize, Deserializer, Serialize};
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum TransactionEvents {
    Request,
    Created,
    Pending,
}

#[derive(Debug, Serialize)]
pub enum TransactionEventMessage {
    Request(RequestTransactionEventPayload),
    Created(CreatedTransactionEventPayload),
    Pending(PendingTransactionEventPayload),
}

impl TransactionEventMessage {
    pub fn from(payload: String) -> Result<Self, serde_json::Error> {
        let default_event: DefaultTransactionEventPayload = serde_json::from_str(&payload)?;
        match default_event.event {
            TransactionEvents::Request => {
                let event = serde_json::from_str::<RequestTransactionEventPayload>(&payload)?;
                Ok(TransactionEventMessage::Request(event))
            }
            TransactionEvents::Created => {
                let event = serde_json::from_str::<CreatedTransactionEventPayload>(&payload)?;
                Ok(TransactionEventMessage::Created(event))
            }
            TransactionEvents::Pending => {
                let event = serde_json::from_str::<PendingTransactionEventPayload>(&payload)?;
                Ok(TransactionEventMessage::Pending(event))
            }
        }
    }
}

impl<'de> Deserialize<'de> for TransactionEvents {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TransactionEventsVisitor;

        impl<'de> Visitor<'de> for TransactionEventsVisitor {
            type Value = TransactionEvents;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing a TransactionEvents")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value {
                    "Requested" | "Transaction.Request" => Ok(TransactionEvents::Request),
                    "Created" | "Transaction.Created" => Ok(TransactionEvents::Created),
                    "Pending" | "Transaction.Pending" => Ok(TransactionEvents::Pending),
                    _ => Err(de::Error::invalid_value(Unexpected::Str(value), &self)),
                }
            }
        }

        deserializer.deserialize_str(TransactionEventsVisitor)
    }
}
impl<'de> Serialize for TransactionEvents {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            TransactionEvents::Request => serializer.serialize_str("Transaction.Request"),
            TransactionEvents::Created => serializer.serialize_str("Transaction.Created"),
            TransactionEvents::Pending => serializer.serialize_str("Transaction.Pending"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultTransactionEventPayload {
    pub event: TransactionEvents,
    pub sender: Uuid,
    pub receiver: Uuid,
    pub amount: f64,
    pub currency: Currencies,
    pub hash: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingTransactionEventPayload {
    #[serde(flatten)]
    pub default: DefaultTransactionEventPayload,
    pub id: Uuid,
    pub status: TransactionStatus,
    pub reason: Option<String>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatedTransactionEventPayload {
    #[serde(flatten)]
    pub default: DefaultTransactionEventPayload,
    pub id: Uuid,
    pub status: TransactionStatus,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestTransactionEventPayload {
    #[serde(flatten)]
    pub default: DefaultTransactionEventPayload,
}

impl From<Transaction> for PendingTransactionEventPayload {
    fn from(transaction: Transaction) -> Self {
        PendingTransactionEventPayload {
            default: DefaultTransactionEventPayload {
                event: TransactionEvents::Pending,
                sender: transaction.sender.unwrap(),
                receiver: transaction.receiver.unwrap(),
                amount: transaction.amount,
                currency: transaction.currency.unwrap(),
                hash: transaction.hash.clone(),
                created_at: transaction.created_at,
            },
            id: transaction.id.unwrap(),
            status: transaction.status.unwrap(),
            reason: transaction.reason.clone(),
            updated_at: transaction.updated_at,
        }
    }
}

impl From<Transaction> for CreatedTransactionEventPayload {
    fn from(transaction: Transaction) -> Self {
        CreatedTransactionEventPayload {
            default: DefaultTransactionEventPayload {
                event: TransactionEvents::Created,
                sender: transaction.sender.unwrap(),
                receiver: transaction.receiver.unwrap(),
                amount: transaction.amount,
                currency: transaction.currency.unwrap(),
                hash: transaction.hash.clone(),
                created_at: transaction.created_at,
            },
            id: transaction.id.unwrap(),
            status: transaction.status.unwrap(),
            updated_at: transaction.updated_at,
        }
    }
}

impl From<Transaction> for RequestTransactionEventPayload {
    fn from(transaction: Transaction) -> Self {
        RequestTransactionEventPayload {
            default: DefaultTransactionEventPayload {
                event: TransactionEvents::Request,
                sender: transaction.sender.unwrap(),
                receiver: transaction.receiver.unwrap(),
                amount: transaction.amount,
                currency: transaction.currency.unwrap(),
                hash: transaction.hash.clone(),
                created_at: transaction.created_at,
            },
        }
    }
}

impl ToTransaction for RequestTransactionEventPayload {
    fn to_transaction(&self) -> Transaction {
        Transaction {
            id: None,
            sender: Option::from(self.default.sender),
            receiver: Option::from(self.default.receiver),
            amount: self.default.amount,
            currency: Option::from(self.default.currency.clone()),
            hash: self.default.hash.clone(),
            status: Option::from(TransactionStatus::Review),
            reason: None,
            created_at: self.default.created_at,
            updated_at: self.default.created_at,
        }
    }
}

impl ToTransaction for CreatedTransactionEventPayload {
    fn to_transaction(&self) -> Transaction {
        Transaction {
            id: Option::from(self.id),
            sender: Option::from(self.default.sender),
            receiver: Option::from(self.default.receiver),
            amount: self.default.amount,
            currency: Option::from(self.default.currency.clone()),
            hash: self.default.hash.clone(),
            status: Option::from(self.status.clone()),
            reason: None,
            created_at: self.default.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl ToTransaction for PendingTransactionEventPayload {
    fn to_transaction(&self) -> Transaction {
        Transaction {
            id: Option::from(self.id),
            sender: Option::from(self.default.sender),
            receiver: Option::from(self.default.receiver),
            amount: self.default.amount,
            currency: Option::from(self.default.currency.clone()),
            hash: self.default.hash.clone(),
            status: Option::from(self.status.clone()),
            reason: self.reason.clone(),
            created_at: self.default.created_at,
            updated_at: self.updated_at,
        }
    }
}
