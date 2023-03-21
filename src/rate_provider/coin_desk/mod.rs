mod coin_desk_rate;

use serde_json;
use crate::rate;
use self::coin_desk_rate::Rate as InternalRate;
use super::ProviderError;
use super::RateProvider;
use super::Currency;

pub struct CoinDesk {}

impl CoinDesk {
    fn convert_to_internal_rate(response: &str) -> Result<InternalRate, ProviderError> {
        let deserialized_result: serde_json::Result<InternalRate> = serde_json::from_str(&response);

        match deserialized_result {
            Ok(deserialized) => Ok(deserialized),
            Err(e) => Err(ProviderError::new(e.to_string())),
        }
    }
}

impl RateProvider for CoinDesk {
    fn get_name() -> &'static str {
        "CoinDesk"
    }
    fn get(currency: Currency) -> Result<rate::Rate, super::ProviderError> {
        if currency != Currency::Bitcoin {
            return Err(ProviderError::new("This provider only support Bitcoin"));
        }
        let response = Self::download("https://api.coindesk.com/v1/bpi/currentprice/EUR.json")?;
        Self::convert(&response)
    }
    fn convert(response: &str) -> Result<rate::Rate, super::ProviderError> {
        let internal_rate = Self::convert_to_internal_rate(response)?;
        Ok(rate::Rate::new(
            Currency::Bitcoin,
            internal_rate.bpi.usd.rate,
            internal_rate.bpi.eur.rate,
        ))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_test() {
        let result: Result<rate::Rate, ProviderError> = <CoinDesk as RateProvider>::get(Currency::Bitcoin);

        assert!(result.is_ok())
    }
}
