mod rate_series;

pub use self::rate_series::RateSeries;
use rate_provider::Currency;
use matrix::CoordinatesTrait;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rate {
    //": "bitcoin",
    pub id: String,
    //": "Bitcoin",
    pub name: String,
    //": "BTC",
    pub symbol: String,
    //": "1.0",
    pub price_btc: f32,
    //": "573.137",
    pub price_usd: f32,
    // "7278.69583695",
    pub price_eur: f32,
}

impl Rate {
    pub fn new(currency: Currency, price_usd: f32, price_eur: f32) -> Self {
        Rate {
            id: currency.name().to_owned().to_lowercase(),
            name: currency.name().to_owned(),
            symbol: currency.symbol().to_owned(),
            price_btc: 1.0,
            price_usd: price_usd.clone(),
            price_eur: price_eur.clone(),
        }
    }
}

impl CoordinatesTrait for Rate {
    fn x(&self) -> usize {
        0
    }

    fn y(&self) -> usize {
        self.price_usd.round() as usize
    }
}