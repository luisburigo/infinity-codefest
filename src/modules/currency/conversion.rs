use rust_decimal::Decimal;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use crate::types::currency::Currencies;

pub trait CurrencyConversion {
    fn convert_to(&self, amount: f64, to: Currencies) -> f64;
}

impl Currencies {
    pub fn to_usd(&self, amount: f64) -> f64 {
        let usd = match self {
            Currencies::USD => 1.00,
            Currencies::BR => 0.20,
            Currencies::BTC => 60000.00,
            Currencies::ETH => 3000.00,
            Currencies::IC => 1000000.00,
        };

        usd * amount
    }
}

impl CurrencyConversion for Currencies {
    fn convert_to(&self, amount: f64, to: Currencies) -> f64 {
        let from_usd = self.to_usd(amount);
        let to_usd = to.to_usd(1.00);
        let value = Decimal::from_f64( from_usd / to_usd).unwrap();
        value.round_dp(2).to_f64().unwrap()
    }
}

pub mod tests {
    use super::*;

    #[test]
    fn test_brl_to_usd() {
        let amount = 100.00;
        let from = Currencies::USD;

        let result = from.convert_to(amount, Currencies::BR);
        assert_eq!(result, 500.00);
    }

    #[test]
    fn test_btc_to_usd() {
        let amount = 1.0;
        let from = Currencies::BTC;

        let result = from.convert_to(amount, Currencies::BR);
        assert_eq!(result, 300000.00);
    }

    #[test]
    fn test_ic_to_usd() {
        let amount = 1.0;
        let from = Currencies::IC;

        let result = from.convert_to(amount, Currencies::BTC);
        assert_eq!(result, 16.67);
    }
}