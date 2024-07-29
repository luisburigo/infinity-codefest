use crate::types::currency::Currencies;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum UserStatus {
    Review,
    Success,
    Failed,
    Approved,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
