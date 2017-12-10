use rate;

pub struct CoinMarketCap {}

impl super::RateProvider for CoinMarketCap {
    fn get_all() -> Option<Vec<rate::Rate>> {
        let response = Self::download("https://api.coinmarketcap.com/v1/ticker/?convert=EUR&limit=10");
        Self::convert_all(&response)
    }

    fn get() -> Option<rate::Rate> {
        unimplemented!()
    }
    fn convert(response: &str) -> Option<rate::Rate> {
        unimplemented!()
    }
}
