use std::fmt;
use serde::{de, Deserialize, Deserializer, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::de::{Unexpected, Visitor};
use crate::types::currency::Currencies;

#[derive(Debug, Clone)]
pub enum UserStatus {
    Review,
    Success,
    Failed,
    Approved,
}

impl<'de> Deserialize<'de> for UserStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UserEventsVisitor;

        impl<'de> Visitor<'de> for UserEventsVisitor {
            type Value = UserStatus;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing a UserStatus")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value {
                    "review" | "Review" => Ok(UserStatus::Review),
                    "success" | "Success" => Ok(UserStatus::Success),
                    "failed" | "Failed" => Ok(UserStatus::Failed),
                    "approved" | "Approved" => Ok(UserStatus::Approved),
                    _ => Err(de::Error::invalid_value(Unexpected::Str(value), &self)),
                }
            }
        }

        deserializer.deserialize_str(UserEventsVisitor)
    }
}

impl<'de> Serialize for UserStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            UserStatus::Review => serializer.serialize_str("review"),
            UserStatus::Approved => serializer.serialize_str("approved"),
            UserStatus::Failed => serializer.serialize_str("failed"),
            UserStatus::Success => serializer.serialize_str("success"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Option<Uuid>,
    pub name: String,
    pub email: String,
    pub public_key: String,
    pub status: Option<UserStatus>,
    pub balance: Option<f64>,
    pub currency: Option<Currencies>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub trait ToUser {
    fn to_user(&self) -> User;
}
