use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use crate::types::currency::Currencies;

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

    pub fn from_usd(&self, amount: f64) -> f64 {
        let usd = match self {
            Currencies::USD => 1.00,
            Currencies::BR => 0.20,
            Currencies::BTC => 60000.00,
            Currencies::ETH => 3000.00,
            Currencies::IC => 1000000.00,
        };

        amount / usd
    }
}

pub mod tests {
    use super::*;

    #[test]
    fn test_convert_usd_brl() {
        let from_amount = 100.00;
        let from = Currencies::USD;

        let to_amount = 1;
        let to = Currencies::BR;

        let result = from.to_usd(from_amount) * to.to_usd(to_amount as f64);

        assert_eq!(result, 200.00);
    }

    #[test]
    fn test_convert_usd_btc() {
        let from_amount = 1.00;
        let from = Currencies::BR;

        let to_amount = 1.00;
        let to = Currencies::BTC;

        let to_usd = to.to_usd(to_amount);
        let from_usd = from.to_usd(from_amount);

        let result = from_usd / to_usd;

        assert_eq!(result, 0.0000033333333333333333);
    }

    #[test]
    fn test_convert_eth_btc() {
        let from_amount = 1.00;
        let from = Currencies::ETH;

        let to_amount = 1.00;
        let to = Currencies::BTC;

        let to_usd = to.to_usd(to_amount);
        let from_usd = from.to_usd(from_amount);

        let result = from_usd / to_usd;

        assert_eq!(result, 0.050000000000000003);
    }

    #[test]
    fn test_convert_ic_eth() {
        let from_amount = 1.00;
        let from = Currencies::IC;

        let to_amount = 1.00;
        let to = Currencies::ETH;

        let to_usd = to.to_usd(to_amount);
        let from_usd = from.to_usd(from_amount);

        let result = from_usd / to_usd;

        assert_eq!(result, 333.3333333333333);
    }
}