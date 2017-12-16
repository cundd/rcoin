mod intermediate_rate;

use serde_json;
use rate;
use self::intermediate_rate::*;
use super::ProviderError;
use super::RateProvider;

pub struct BlockchainInfo {}

impl BlockchainInfo {
    fn convert_to_internal_rates(response: &str) -> Result<CurrencyRateMap, ProviderError> {
        let deserialized_result: serde_json::Result<CurrencyRateMap> = serde_json::from_str(&response);

        match deserialized_result {
            Ok(deserialized) => Ok(deserialized),
            Err(e) => Err(ProviderError::new(e.to_string())),
        }
    }
}

impl RateProvider for BlockchainInfo {
    fn get() -> Result<rate::Rate, ProviderError> {
        let response = Self::download("https://blockchain.info/ticker")?;
        Self::convert(&response)
    }

    fn convert(response: &str) -> Result<rate::Rate, ProviderError> {
        let currency_rates: CurrencyRateMap = Self::convert_to_internal_rates(response)?;

        Ok(rate::Rate::new(
            match currency_rates.get("USD") {
                Some(rate) => rate.last,
                None => 0.0,
            },
            match currency_rates.get("EUR") {
                Some(rate) => rate.last,
                None => 0.0,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_test() {
        let result: Result<rate::Rate, ProviderError> = <BlockchainInfo as RateProvider>::get();

        assert!(result.is_ok())
    }
}
