use std::fmt;
use serde::{de, Deserialize, Deserializer, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::de::{Unexpected, Visitor};
use crate::types::currency::Currencies;
use crate::types::user::types::{ToUser, User, UserStatus};

#[derive(Debug)]
pub enum UserEventError {
    Amiquip(amiquip::Error),
    Serde(serde_json::Error),
}

impl From<amiquip::Error> for UserEventError {
    fn from(err: amiquip::Error) -> UserEventError {
        UserEventError::Amiquip(err)
    }
}

impl From<serde_json::Error> for UserEventError {
    fn from(err: serde_json::Error) -> UserEventError {
        UserEventError::Serde(err)
    }
}

#[derive(Debug)]
pub enum UserEventMessage {
    Requested(RequestedUserEventPayload),
    Created(CreatedUserEventPayload),
    Pending(PendingUserEventPayload),
}

impl UserEventMessage {
    pub fn from(payload: String) -> Result<Self, serde_json::Error> {
        let default_event: DefaultUserEventPayload = serde_json::from_str(&payload)?;
        match default_event.event {
            UserEvents::Requested => {
                let event = serde_json::from_str::<RequestedUserEventPayload>(&payload)?;
                Ok(UserEventMessage::Requested(event))
            },
            UserEvents::Created => {
                let event = serde_json::from_str::<CreatedUserEventPayload>(&payload)?;
                Ok(UserEventMessage::Created(event))
            },
            UserEvents::Pending => {
                let event = serde_json::from_str::<PendingUserEventPayload>(&payload)?;
                Ok(UserEventMessage::Pending(event))
            },
        }
    }
}

impl<'de> Deserialize<'de> for UserEvents {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UserEventsVisitor;

        impl<'de> Visitor<'de> for UserEventsVisitor {
            type Value = UserEvents;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing a UserEvent")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value {
                    "Requested" | "User.Requested" => Ok(UserEvents::Requested),
                    "Created" | "User.Created" => Ok(UserEvents::Created),
                    "Pending" | "User.Pending" => Ok(UserEvents::Pending),
                    _ => Err(de::Error::invalid_value(Unexpected::Str(value), &self)),
                }
            }
        }

        deserializer.deserialize_str(UserEventsVisitor)
    }
}

#[derive(Debug, Serialize)]
pub enum UserEvents {
    Requested,
    Created,
    Pending,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultUserEventPayload {
    pub id: Option<Uuid>,
    pub status: Option<UserStatus>,
    pub event: UserEvents,
    pub name: String,
    pub email: String,
    pub public_key: String,
    pub balance: Option<u8>,
    pub currency: Option<Currencies>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PendingUserEventPayload {
    #[serde(flatten)]
    pub default: DefaultUserEventPayload,
    pub reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatedUserEventPayload {
    #[serde(flatten)]
    pub default: DefaultUserEventPayload,
    pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestedUserEventPayload {
    #[serde(flatten)]
    pub default: DefaultUserEventPayload,
}

impl ToUser for RequestedUserEventPayload {
    fn to_user(&self) -> User {
        User {
            id: self.default.id,
            balance: self.default.balance,
            created_at: self.default.created_at,
            currency: self.default.currency.clone(),
            email: self.default.email.clone(),
            name: self.default.name.clone(),
            status: self.default.status.clone(),
            updated_at: self.default.updated_at,
            public_key: self.default.public_key.clone(),
        }
    }
}

impl ToUser for CreatedUserEventPayload  {
    fn to_user(&self) -> User {
        User {
            id: Some(self.id),
            balance: self.default.balance,
            created_at: self.default.created_at,
            currency: self.default.currency.clone(),
            email: self.default.email.clone(),
            name: self.default.name.clone(),
            status: self.default.status.clone(),
            updated_at: self.default.updated_at,
            public_key: self.default.public_key.clone(),
        }
    }
}

impl ToUser for PendingUserEventPayload  {
    fn to_user(&self) -> User {
        User {
            id: self.default.id,
            balance: self.default.balance,
            created_at: self.default.created_at,
            currency: self.default.currency.clone(),
            email: self.default.email.clone(),
            name: self.default.name.clone(),
            status: self.default.status.clone(),
            updated_at: self.default.updated_at,
            public_key: self.default.public_key.clone(),
        }
    }
}