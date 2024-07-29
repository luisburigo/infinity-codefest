use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::types::currency::Currencies;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UserStatus {
    Review,
    Success,
    Failed,
    Approved,
}

#[derive(Debug)]
pub struct User {
    pub id: Option<Uuid>,
    pub name: String,
    pub email: String,
    pub public_key: String,
    pub status: Option<UserStatus>,
    pub balance: Option<u8>,
    pub currency: Option<Currencies>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub trait ToUser {
    fn to_user(&self) -> User;
}