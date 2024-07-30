use std::fmt;
use serde::{de, Deserialize, Deserializer, Serialize};
use serde::de::{Unexpected, Visitor};

#[derive(Debug, Serialize, Clone)]
pub enum Currencies {
    IC,
    USD,
    BR,
    ETH,
    BTC,
}

impl std::fmt::Display for Currencies {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Currencies::IC => write!(f, "IC"),
            Currencies::USD => write!(f, "USD"),
            Currencies::BR => write!(f, "BR"),
            Currencies::ETH => write!(f, "ETH"),
            Currencies::BTC => write!(f, "BTC"),
        }
    }
}

impl<'de> Deserialize<'de> for Currencies {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CurrencyVisitor;

        impl<'de> Visitor<'de> for CurrencyVisitor {
            type Value = Currencies;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing a Currencies")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value {
                    "IC" | "ic" => Ok(Currencies::IC),
                    "USD" | "usd" => Ok(Currencies::USD),
                    "BR" | "br" => Ok(Currencies::BR),
                    "ETH" | "eth" => Ok(Currencies::ETH),
                    "BTC" | "btc" => Ok(Currencies::BTC),
                    _ => Err(de::Error::invalid_value(Unexpected::Str(value), &self)),
                }
            }
        }

        deserializer.deserialize_str(CurrencyVisitor)
    }
}