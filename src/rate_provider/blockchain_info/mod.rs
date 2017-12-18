mod intermediate_rate;

use serde_json;
use rate;
use self::intermediate_rate::*;
use super::ProviderError;
use super::RateProvider;
use super::Currency;

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
    fn get_name() -> &'static str {
        "BlockchainInfo"
    }

    fn get(currency: Currency) -> Result<rate::Rate, ProviderError> {
        if currency != Currency::Bitcoin {
            return Err(ProviderError::new("This provider only support Bitcoin"));
        }
        let response = Self::download("https://blockchain.info/ticker")?;
        Self::convert(&response)
    }

    fn convert(response: &str) -> Result<rate::Rate, ProviderError> {
        let currency_rates: CurrencyRateMap = Self::convert_to_internal_rates(response)?;

        Ok(rate::Rate::new(
            Currency::Bitcoin,
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
        let result: Result<rate::Rate, ProviderError> = <BlockchainInfo as RateProvider>::get(Currency::Bitcoin);

        assert!(result.is_ok())
    }
}
