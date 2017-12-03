use rate::Rate;
use super::RateProvider;

pub struct CoinMarketCap {}

impl RateProvider for CoinMarketCap {
    fn get() -> Option<Vec<Rate>> {
        let response = Self::download("https://api.coinmarketcap.com/v1/ticker/?convert=EUR&limit=10");
        Self::convert(&response)
    }
}
