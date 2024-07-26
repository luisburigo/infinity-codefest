use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Currencies {
    IC,
    USD,
    BRL,
    ETH,
    BTC,
}