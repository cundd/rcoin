// https://api.cryptonator.com/api/ticker/ltc-usd

//{
//    "ticker": {
//        "base": "LTC",
//        "target": "USD",
//        "price": "248.16084013",
//        "volume": "286939.69094764",
//        "change": "-0.67224984"
//    },
//    "timestamp": 1515878702,
//    "success": true,
//    "error": ""
//}

mod intermediate_rate;

use serde_json;
use rate;
use super::ProviderError;
use super::RateProvider;
use super::Currency;
use self::intermediate_rate::*;

pub struct Cryptonator {}

impl Cryptonator {
    fn convert_to_internal_rate(response: &str) -> Result<IntermediateRate, ProviderError> {
        let deserialized_result: serde_json::Result<IntermediateRate> = serde_json::from_str(&response);

        match deserialized_result {
            Ok(deserialized) => Ok(deserialized),
            Err(e) => Err(ProviderError::new(e.to_string())),
        }
    }

    fn download_pair(crypto_currency: &Currency, fiat_currency: &str) -> Result<String, ProviderError> {
        Self::download(&format!(
            "https://api.cryptonator.com/api/ticker/{}-{}",
            crypto_currency.symbol(),
            fiat_currency.to_lowercase()
        ))
    }

    fn get_pair_in_internal_rate(crypto_currency: &Currency, fiat_currency: &str) -> Result<IntermediateRate, ProviderError> {
        Self::convert_to_internal_rate(&Self::download_pair(crypto_currency, fiat_currency)?)
    }
}

impl RateProvider for Cryptonator {
    fn get_name() -> &'static str {
        "Cryptonator"
    }
    fn get(currency: Currency) -> Result<rate::Rate, ProviderError> {
        let usd_rate:IntermediateRate = Self::get_pair_in_internal_rate(&currency, "usd")?;
        let eur_rate:IntermediateRate = Self::get_pair_in_internal_rate(&currency, "eur")?;

        Ok(rate::Rate::new(currency, usd_rate.price(), eur_rate.price()))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_test() {
        let result: Result<rate::Rate, ProviderError> = <Cryptonator as RateProvider>::get(Currency::Bitcoin);

        assert!(result.is_ok())
    }
}