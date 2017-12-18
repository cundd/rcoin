mod intermediate_rate;

use serde_json;
use rate;
use super::ProviderError;
use super::RateProvider;
use super::Currency;
use self::intermediate_rate::*;

pub struct CoinMarketCap {}

impl CoinMarketCap {
    fn convert_to_internal_rates(response: &str) -> Result<IntermediateRateCollection, ProviderError> {
        let deserialized_result: serde_json::Result<IntermediateRateCollection> = serde_json::from_str(&response);

        match deserialized_result {
            Ok(deserialized) => Ok(deserialized),
            Err(e) => Err(ProviderError::new(e.to_string())),
        }
    }
}

impl RateProvider for CoinMarketCap {
    fn get_name() -> &'static str {
        "CoinMarketCap"
    }
    fn get_all() -> Result<Vec<rate::Rate>, ProviderError> {
        let response = Self::download("https://api.coinmarketcap.com/v1/ticker/?convert=EUR&limit=10")?;
        Self::convert_all(&response)
    }

    fn get(currency: Currency) -> Result<rate::Rate, ProviderError> {
        let all = CoinMarketCap::get_all()?;

        match all.into_iter()
            .find(|rate| {
                let current_currency = Currency::new(rate.symbol.as_str());
                if let Some(current_currency) = current_currency {
                    return current_currency == currency;
                }
                false
            }) {
            Some(rate) => Ok(rate),
            None => Err(ProviderError::new(format!("No rate for currency {} found", currency.name()))),
        }
    }

    #[allow(unused)]
    fn convert(response: &str) -> Result<rate::Rate, ProviderError> { unimplemented!() }
    fn convert_all(response: &str) -> Result<Vec<rate::Rate>, ProviderError> {
        let internal_rates = Self::convert_to_internal_rates(response)?;
        let mut rates = Vec::with_capacity(internal_rates.len());

        for internal_rate in internal_rates {
            rates.push(rate::Rate {
                id: internal_rate.id,
                name: internal_rate.name,
                symbol: internal_rate.symbol,
                price_btc: internal_rate.price_btc.parse().unwrap_or(0.0),
                price_usd: internal_rate.price_usd.parse().unwrap_or(0.0),
                price_eur: internal_rate.price_eur.parse().unwrap_or(0.0),
            });
        }

        Ok(rates)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_test() {
        let result: Result<rate::Rate, ProviderError> = <CoinMarketCap as RateProvider>::get(Currency::Bitcoin);

        assert!(result.is_ok())
    }
}
