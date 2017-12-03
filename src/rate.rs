#[derive(Serialize, Deserialize, Debug)]
pub struct Rate {
    //": "bitcoin",
    pub id: String,
    //": "Bitcoin",
    pub name: String,
    //": "BTC",
    pub symbol: String,
    //": "1.0",
    pub price_btc: String,
    //": "573.137",
    pub price_usd: String,
    // "7278.69583695",
    pub price_eur: String,
}

impl Rate {
    pub fn new(price_usd: &str, price_eur: &str) -> Self {
        Rate {
            id: "bitcoin".to_owned(),
            name: "Bitcoin".to_owned(),
            symbol: "BTC".to_owned(),
            price_btc: "1.0".to_owned(),
            price_usd: price_usd.to_string(),
            price_eur: price_eur.to_string(),
        }
    }
}
