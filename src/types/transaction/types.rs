use crate::types::currency::Currencies;
use chrono::{DateTime, Utc};
use serde::de::{Unexpected, Visitor};
use serde::{de, Deserialize, Deserializer, Serialize};
use std::fmt;
use k256::ecdsa::{Signature, VerifyingKey};
use k256::ecdsa::signature::Verifier;
use tiny_keccak::{Hasher, Keccak};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum TransactionStatus {
    Review,
    Success,
    Failed,
    Approved,
}

impl<'de> Deserialize<'de> for TransactionStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TransactionStatusVisitor;

        impl<'de> Visitor<'de> for TransactionStatusVisitor {
            type Value = TransactionStatus;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing a TransactionStatus")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value {
                    "review" | "Review" => Ok(TransactionStatus::Review),
                    "success" | "Success" => Ok(TransactionStatus::Success),
                    "failed" | "Failed" => Ok(TransactionStatus::Failed),
                    "approved" | "Approved" => Ok(TransactionStatus::Approved),
                    _ => Err(de::Error::invalid_value(Unexpected::Str(value), &self)),
                }
            }
        }

        deserializer.deserialize_str(TransactionStatusVisitor)
    }
}

impl<'de> Serialize for TransactionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            TransactionStatus::Review => serializer.serialize_str("review"),
            TransactionStatus::Approved => serializer.serialize_str("approved"),
            TransactionStatus::Failed => serializer.serialize_str("failed"),
            TransactionStatus::Success => serializer.serialize_str("success"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TxHash {
    String(String),
    // @TODO: How would be a hash type here?
    // Hash(dyn Hash)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub id: Option<Uuid>,
    pub sender: Option<Uuid>,
    pub receiver: Option<Uuid>,
    pub amount: f64,
    pub currency: Option<Currencies>,
    pub hash: String,
    pub status: Option<TransactionStatus>,
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Transaction {
    pub fn verify_signature(&self, public_key: String) -> Result<bool, Box<dyn std::error::Error>> {
        let public_key_bytes = hex::decode(public_key)?;
        let verifying_key = VerifyingKey::from_sec1_bytes(&public_key_bytes)?;

        let signature_bytes = hex::decode(&self.hash)?;
        let signature = Signature::from_der(&signature_bytes)?;

        let calculated_hash = self.calculate_hash();
        let calculated_hash_bytes = hex::decode(calculated_hash)?;

        verifying_key.verify(calculated_hash_bytes.as_slice(), &signature).map(|_| true).map_err(Into::into)
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Keccak::v256();

        if let Some(sender) = &self.sender {
            hasher.update(sender.as_bytes());
        }
        if let Some(receiver) = &self.receiver {
            hasher.update(receiver.as_bytes());
        }

        hasher.update(&self.amount.to_le_bytes());
        hasher.update(self.created_at.to_rfc3339().as_bytes());

        if let Some(currency) = &self.currency {
            hasher.update(currency.as_ref());
        }

        let mut output = [0u8; 32];
        hasher.finalize(&mut output);
        hex::encode(output)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransactionIdentifiers {
    pub user_id: Uuid,
    pub id: Uuid,
    pub sender: Uuid,
}

#[derive(Debug, Serialize)]
pub struct TransactionsByType {
    pub user_id: Uuid,
    pub count: i32,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTransactionPayload {
    pub user_id: Uuid,
    pub transaction: Transaction,
}

pub trait ToTransaction {
    fn to_transaction(&self) -> Transaction;
}
