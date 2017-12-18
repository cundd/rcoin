mod intermediate_rate;

use serde_json;
use rate;
use super::ProviderError;
use super::RateProvider;
use super::Currency;
use self::intermediate_rate::*;

pub struct CryptoCompare {}

impl CryptoCompare {
    fn convert_to_internal_rate(response: &str) -> Result<IntermediateRate, ProviderError> {
        let deserialized_result: serde_json::Result<IntermediateRate> = serde_json::from_str(&response);

        match deserialized_result {
            Ok(deserialized) => Ok(deserialized),
            Err(e) => Err(ProviderError::new(e.to_string())),
        }
    }
}

impl RateProvider for CryptoCompare {
    fn get_name() -> &'static str {
        "CryptoCompare"
    }
    fn get(currency: Currency) -> Result<rate::Rate, ProviderError> {
        let response = Self::download(&format!("https://min-api.cryptocompare.com/data/price?fsym={}&tsyms=USD,EUR", currency.symbol()))?;
        let internal_rate = Self::convert_to_internal_rate(&response)?;

        Ok(rate::Rate::new(currency, internal_rate.usd, internal_rate.eur))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_test() {
        let result: Result<rate::Rate, ProviderError> = <CryptoCompare as RateProvider>::get(Currency::Bitcoin);

        assert!(result.is_ok())
    }
}