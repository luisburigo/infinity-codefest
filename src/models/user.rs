use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub enum Currencies {
    IC,
    USD,
    BRL,
    ETH,
    BTC,
}

impl Currencies {
    pub fn as_str(&self) -> &str {
        // works like a swtich in typescript.
        // if match self(the currency) === IC return "IC"
        match self {
            Currencies::IC => "IC",
            Currencies::USD => "usd",
            Currencies::BRL => "brl",
            Currencies::ETH => "eth",
            Currencies::BTC => "btc",
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum UserStatus {
    REVIEW,
    SUCCESS,
    FAILED,
    APPROVED,
}

impl UserStatus {
    pub fn enum_to_string(&self) -> &str {
        match self {
            UserStatus::REVIEW => "review",
            UserStatus::SUCCESS => "success",
            UserStatus::FAILED => "failed",
            UserStatus::APPROVED => "approved",
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    name: String,
    email: String,
    public_key: String,
    status: UserStatus,
    balance: f64,
    currency: Currencies,
}
