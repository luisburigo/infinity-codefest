use std::fmt;
use serde::{de, Deserialize, Deserializer, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::de::{Unexpected, Visitor};
use serde_json::Value;
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

#[derive(Debug, Serialize)]
pub enum UserEventMessage {
    Request(RequestedUserEventPayload),
    Created(CreatedUserEventPayload),
    Pending(PendingUserEventPayload),
}

fn clean_duplicate_id(json_str: &str) -> Result<String, serde_json::Error> {
    let mut value: Value = serde_json::from_str(json_str)?;
    if let Some(obj) = value.as_object_mut() {
        let id = obj.remove("id");
        if let Some(id_value) = id {
            obj.insert("id".to_owned(), id_value);
        }
    }
    serde_json::to_string(&value)
}

impl UserEventMessage {
    pub fn from(payload: String) -> Result<Self, serde_json::Error> {
        let cleaned_payload = clean_duplicate_id(&payload)?;
        let default_event: DefaultUserEventPayload = serde_json::from_str(&cleaned_payload)?;
        match default_event.event {
            UserEvents::Request => {
                let event = serde_json::from_str::<RequestedUserEventPayload>(&cleaned_payload)?;
                Ok(UserEventMessage::Request(event))
            },
            UserEvents::Created => {
                let event = serde_json::from_str::<CreatedUserEventPayload>(&cleaned_payload)?;
                Ok(UserEventMessage::Created(event))
            },
            UserEvents::Pending => {
                let event = serde_json::from_str::<PendingUserEventPayload>(&cleaned_payload)?;
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
                    "Requested" | "User.Request" => Ok(UserEvents::Request),
                    "Created" | "User.Created" => Ok(UserEvents::Created),
                    "Pending" | "User.Pending" => Ok(UserEvents::Pending),
                    _ => Err(de::Error::invalid_value(Unexpected::Str(value), &self)),
                }
            }
        }

        deserializer.deserialize_str(UserEventsVisitor)
    }
}
impl<'de> Serialize for UserEvents {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            UserEvents::Request => serializer.serialize_str("User.Request"),
            UserEvents::Created => serializer.serialize_str("User.Created"),
            UserEvents::Pending => serializer.serialize_str("User.Pending"),
        }
    }
}

#[derive(Debug)]
pub enum UserEvents {
    Request,
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
    pub balance: Option<f64>,
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

impl From<User> for PendingUserEventPayload {
    fn from(user: User) -> PendingUserEventPayload {
        PendingUserEventPayload {
            default: DefaultUserEventPayload {
                id: user.id,
                balance: user.balance,
                created_at: user.created_at,
                currency: user.currency.clone(),
                email: user.email.clone(),
                name: user.name.clone(),
                status: user.status.clone(),
                updated_at: user.updated_at,
                event: UserEvents::Pending,
                public_key: user.public_key.clone(),
            },
            reason: None,
        }
    }
}

impl From<User> for CreatedUserEventPayload {
    fn from(user: User) -> CreatedUserEventPayload {
        CreatedUserEventPayload {
            default: DefaultUserEventPayload {
                id: user.id,
                balance: user.balance,
                created_at: user.created_at,
                currency: user.currency.clone(),
                email: user.email.clone(),
                name: user.name.clone(),
                status: user.status.clone(),
                updated_at: user.updated_at,
                event: UserEvents::Created,
                public_key: user.public_key.clone(),
            },
            id: user.id.unwrap(),
        }
    }
}